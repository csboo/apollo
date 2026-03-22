#!/usr/bin/env fish

if ! set -q apollo_srv_addr
    set apollo_srv_addr "http://127.0.0.1:8080"
end
set apollo_api_base "$apollo_srv_addr/api"
echo "server is at: $apollo_api_base"

function join -d "<username> (-b <cookie-load> -c <cookie-save>) [...]"
    POST join -d "{\"username\":\"$argv[1]\"}" $argv[2..]
end

function set_solution -d "<puzzle_id> <solution> <password> (-b <cookie-load> -c <cookie-save>) [...]"
    POST set_solution -d "{\"puzzle_solutions\":{\"$argv[1]\":{\"solution\":\"$argv[2]\",\"value\":32}},\"password\":\"$argv[3]\"}" $argv[4..]
end

function auth_state -d "(-b <cookie-load> -c <cookie-save>) [...]"
    GET auth_state $argv
end

function submit -d "<puzzle_id> <solution> (-b <cookie-load> -c <cookie-save>) [...]"
    POST submit -d "{\"puzzle_id\":\"$argv[1]\",\"solution\":\"$argv[2]\"}" $argv[3..]
end

function handle_resp
    set resp "$argv[1]"
    if ! test "$resp" = ""
        echo "$resp" | jq -r
        or begin
            echo "$resp" && return 1
        end
    else
        echo "WARN: got empty response"
    end
end

function GET
    set resp (curl --silent "$apollo_api_base/$argv[1]" $argv[2..])
    handle_resp "$resp"
end

function POST
    set resp (curl --silent -XPOST "$apollo_api_base/$argv[1]" $argv[2..])
    handle_resp "$resp"
end

function mock_solutions -d "<from> <to> <password>"
    for id in (seq $argv[1] $argv[2])
        set_solution $id (math "$id * 10") $argv[3] &
    end
end

function mock_join -d "<team-count> <puzzle-count>"
    petname --count $argv[1] | while read -l the_petname
        join $the_petname
        mock_solve $argv[2] &
    end
end

function mock_solve -d "<puzzle-count>"
    for id in (seq 0 $argv[1])
        set solution (math "$id * 10 + $(random) % 2")
        echo "id: $id => solution: $solution"
        submit $id $solution
    end
end

function set_admin_password -d "<password>"
    if test (count $argv) -lt 1
        echo "password not provided"
        return 1
    end
    POST set_admin_password -d "{\"password\":\"$argv[1]\"}"
end

switch "$argv[1]"
    case "" help --help -h
        echo
        echo "usage: fish $(status filename) <command> [args...]"
        echo
        echo "commands:"

        for line in (grep '[f]unction .* -d ' (status filename))
            set name (string replace -r '.*function (\S+).*' '$1' -- $line)
            set desc (string replace -r '.*-d "(.+)"' '$1' -- $line)
            printf "- %-20s %s\n" $name $desc
        end

    case "*"
        $argv
end
