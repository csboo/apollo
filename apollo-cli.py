#!/usr/bin/env python3
import json
import os
import string
import sys
import urllib.error
import urllib.request
from http.cookies import SimpleCookie


API_BASE = os.environ.get("APOLLO_SERVER", "http://127.0.0.1:8080").rstrip("/") + "/api"
DEFAULT_PUZZLE_VALUE = 32


def usage():
    print(
        "usage: apollo-cli.py <command> [args...]\n"
        "\n"
        "set_admin_password <password>\n"
        "create_puzzle <id> <solution> <password>\n"
        "join <username>\n"
        "auth_state <username>\n"
        "submit <username> <id> <solution>\n"
        "logout <username>\n"
        "mock_puzzles <from_id_int> <to_id_int> <password>\n"
        "mock_join <from_char> <to_char>",
        file=sys.stderr,
    )


def cookie_path(username):
    return os.path.join(".", f".{username}-apollo-sid.cookie")


def save_user_cookie(username, sid):
    with open(cookie_path(username), "w", encoding="utf-8") as f:
        f.write(sid)


def load_user_cookie(username):
    path = cookie_path(username)
    try:
        with open(path, "r", encoding="utf-8") as f:
            sid = f.read().strip()
    except FileNotFoundError:
        print(f"missing cookie file: {path}", file=sys.stderr)
        raise SystemExit(1)
    if not sid:
        print(f"empty cookie file: {path}", file=sys.stderr)
        raise SystemExit(1)
    return sid


def delete_user_cookie(username):
    path = cookie_path(username)
    if os.path.exists(path):
        os.remove(path)


def print_response_text(text):
    if text == "":
        return
    try:
        data = json.loads(text)
    except json.JSONDecodeError:
        print(text)
        return
    if isinstance(data, str):
        print(data)
    else:
        print(json.dumps(data, ensure_ascii=False, indent=2))


def request_json(method, path, payload=None, sid=None):
    body = None
    headers = {}
    if payload is not None:
        body = json.dumps(payload).encode("utf-8")
        headers["Content-Type"] = "application/json"
    if sid:
        headers["Cookie"] = f"sid={sid}"

    req = urllib.request.Request(
        API_BASE + path, data=body, headers=headers, method=method
    )
    try:
        with urllib.request.urlopen(req) as resp:
            return resp.status, resp.headers, resp.read().decode("utf-8")
    except urllib.error.HTTPError as err:
        text = err.read().decode("utf-8")
        if text:
            print_response_text(text)
        else:
            print(f"HTTP {err.code}", file=sys.stderr)
        raise SystemExit(1)
    except urllib.error.URLError as err:
        print(f"request failed: {err.reason}", file=sys.stderr)
        raise SystemExit(1)


def extract_sid(set_cookie_headers):
    for header in set_cookie_headers:
        cookie = SimpleCookie()
        cookie.load(header)
        if "sid" in cookie:
            return cookie["sid"].value
    print("missing sid cookie in response", file=sys.stderr)
    raise SystemExit(1)


def cmd_set_admin_password(args):
    if len(args) != 1:
        usage()
        raise SystemExit(1)
    _, _, text = request_json("POST", "/set_admin_password", {"password": args[0]})
    print_response_text(text)


def cmd_create_puzzle(args):
    if len(args) != 3:
        usage()
        raise SystemExit(1)
    puzzle_id, solution, password = args
    payload = {
        "puzzle_solutions": {
            puzzle_id: {
                "solution": solution,
                "value": DEFAULT_PUZZLE_VALUE,
            }
        },
        "password": password,
    }
    _, _, text = request_json("POST", "/set_solution", payload)
    print_response_text(text)


def cmd_join(args):
    if len(args) != 1:
        usage()
        raise SystemExit(1)
    username = args[0]
    _, headers, text = request_json("POST", "/join", {"username": username})
    sid = extract_sid(headers.get_all("Set-Cookie", []))
    save_user_cookie(username, sid)
    print_response_text(text)


def cmd_auth_state(args):
    if len(args) != 1:
        usage()
        raise SystemExit(1)
    sid = load_user_cookie(args[0])
    _, _, text = request_json("GET", "/auth_state", sid=sid)
    print_response_text(text)


def cmd_submit(args):
    if len(args) != 3:
        usage()
        raise SystemExit(1)
    username, puzzle_id, solution = args
    sid = load_user_cookie(username)
    payload = {"puzzle_id": puzzle_id, "solution": solution}
    _, _, text = request_json("POST", "/submit", payload, sid=sid)
    print_response_text(text)


def cmd_logout(args):
    if len(args) != 1:
        usage()
        raise SystemExit(1)
    username = args[0]
    sid = load_user_cookie(username)
    _, _, text = request_json("POST", "/logout", {}, sid=sid)
    delete_user_cookie(username)
    print_response_text(text)


def cmd_mock_puzzles(args):
    if len(args) != 3:
        usage()
        raise SystemExit(1)
    start, end, password = args
    start_int = int(start)
    end_int = int(end)
    for current in range(start_int, end_int + 1):
        cmd_create_puzzle([str(current), str(current * 10), password])


def cmd_mock_join(args):
    if len(args) != 2:
        usage()
        raise SystemExit(1)
    start_char, end_char = args
    if (
        len(start_char) != 1
        or len(end_char) != 1
        or start_char not in string.ascii_lowercase
        or end_char not in string.ascii_lowercase
    ):
        print("mock_join expects lowercase ascii characters", file=sys.stderr)
        raise SystemExit(1)
    if start_char > end_char:
        print("mock_join expects <from_char> <= <to_char>", file=sys.stderr)
        raise SystemExit(1)
    start_idx = string.ascii_lowercase.index(start_char)
    end_idx = string.ascii_lowercase.index(end_char)
    for username in string.ascii_lowercase[start_idx : end_idx + 1]:
        cmd_join([f"user-{username}"])


COMMANDS = {
    "set_admin_password": cmd_set_admin_password,
    "create_puzzle": cmd_create_puzzle,
    "join": cmd_join,
    "auth_state": cmd_auth_state,
    "submit": cmd_submit,
    "logout": cmd_logout,
    "mock_puzzles": cmd_mock_puzzles,
    "mock_join": cmd_mock_join,
}


def main():
    if len(sys.argv) < 2:
        usage()
        raise SystemExit(1)

    command = sys.argv[1]
    handler = COMMANDS.get(command)
    if handler is None:
        usage()
        raise SystemExit(1)

    handler(sys.argv[2:])


if __name__ == "__main__":
    main()
