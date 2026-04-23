#!/usr/bin/env python3
import argparse
import json
import os
import string
import urllib.error
import urllib.request
from http.cookies import SimpleCookie


API_BASE = os.environ.get("APOLLO_SERVER", "http://127.0.0.1:8080").rstrip("/") + "/api"
DEFAULT_PUZZLE_VALUE = 32
PROG = "apollo-cli.py"


def cli_error(message):
    raise SystemExit(f"{PROG}: error: {message}")


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
        cli_error(f"missing cookie file: {path}")
    if not sid:
        cli_error(f"empty cookie file: {path}")
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
            cli_error(f"HTTP {err.code}")
        raise SystemExit(1)
    except urllib.error.URLError as err:
        cli_error(f"request failed: {err.reason}")


def extract_sid(set_cookie_headers):
    for header in set_cookie_headers:
        cookie = SimpleCookie()
        cookie.load(header)
        if "sid" in cookie:
            return cookie["sid"].value
    cli_error("missing sid cookie in response")


def cmd_set_admin_password(args):
    _, _, text = request_json(
        "POST",
        "/set_admin_password",
        {"init_password": args.initial_password, "password": args.password},
    )
    print_response_text(text)


def cmd_create_puzzle(args):
    payload = {
        "puzzle_solutions": {
            args.id: {
                "solution": args.solution,
                "value": DEFAULT_PUZZLE_VALUE,
            }
        },
        "password": args.password,
    }
    _, _, text = request_json("POST", "/set_solution", payload)
    print_response_text(text)


def cmd_join(args):
    _, headers, text = request_json("POST", "/join", {"username": args.username})
    sid = extract_sid(headers.get_all("Set-Cookie", []))
    save_user_cookie(args.username, sid)
    print_response_text(text)


def cmd_auth_state(args):
    sid = load_user_cookie(args.username)
    _, _, text = request_json("GET", "/auth_state", sid=sid)
    print_response_text(text)


def cmd_submit(args):
    sid = load_user_cookie(args.username)
    payload = {"puzzle_id": args.id, "solution": args.solution}
    _, _, text = request_json("POST", "/submit", payload, sid=sid)
    print_response_text(text)


def cmd_logout(args):
    sid = load_user_cookie(args.username)
    _, _, text = request_json("POST", "/logout", {}, sid=sid)
    delete_user_cookie(args.username)
    print_response_text(text)


def cmd_mock_puzzles(args):
    for current in range(args.from_id_int, args.to_id_int + 1):
        cmd_create_puzzle(
            argparse.Namespace(
                id=str(current), solution=str(current * 10), password=args.password
            )
        )


def cmd_mock_join(args):
    if args.from_char > args.to_char:
        cli_error("mock_join expects <from_char> <= <to_char>")
    start_idx = string.ascii_lowercase.index(args.from_char)
    end_idx = string.ascii_lowercase.index(args.to_char)
    for username in string.ascii_lowercase[start_idx : end_idx + 1]:
        cmd_join(argparse.Namespace(username=f"user-{username}"))


def lower_char(value):
    if len(value) != 1 or value not in string.ascii_lowercase:
        raise argparse.ArgumentTypeError("expected a lowercase ascii character")
    return value


def build_parser():
    parser = argparse.ArgumentParser(prog=PROG)
    subparsers = parser.add_subparsers(dest="command", required=True)

    p = subparsers.add_parser("set_admin_password")
    p.add_argument("initial_password")
    p.add_argument("password")
    p.set_defaults(func=cmd_set_admin_password)

    p = subparsers.add_parser("create_puzzle")
    p.add_argument("id")
    p.add_argument("solution")
    p.add_argument("password")
    p.set_defaults(func=cmd_create_puzzle)

    p = subparsers.add_parser("join")
    p.add_argument("username")
    p.set_defaults(func=cmd_join)

    p = subparsers.add_parser("auth_state")
    p.add_argument("username")
    p.set_defaults(func=cmd_auth_state)

    p = subparsers.add_parser("submit")
    p.add_argument("username")
    p.add_argument("id")
    p.add_argument("solution")
    p.set_defaults(func=cmd_submit)

    p = subparsers.add_parser("logout")
    p.add_argument("username")
    p.set_defaults(func=cmd_logout)

    p = subparsers.add_parser("mock_puzzles")
    p.add_argument("from_id_int", type=int)
    p.add_argument("to_id_int", type=int)
    p.add_argument("password")
    p.set_defaults(func=cmd_mock_puzzles)

    p = subparsers.add_parser("mock_join")
    p.add_argument("from_char", type=lower_char)
    p.add_argument("to_char", type=lower_char)
    p.set_defaults(func=cmd_mock_join)

    return parser


def main():
    parser = build_parser()
    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
