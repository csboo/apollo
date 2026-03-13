#!/usr/bin/env fish

if ! set -q apollo_srv_addr
    set apollo_srv_addr "http://127.0.0.1:8080"
end
set apollo_api_base "$apollo_srv_addr/api"
echo "server is at: $apollo_api_base"

function apollo_join -d "<username> [curl args]"
    POST join -d "{\"username\":\"$argv[1]\"}" $argv[1..]
end

function apollo_set_solution -d "<puzzle_id> <solution> <password> [curl args]"
    POST set_solution -d "{\"puzzle_solutions\":{\"$argv[1]\":{\"solution\":\"$argv[2]\",\"value\":32}},\"password\":\"$argv[3]\"}" $argv[4..]
end

function apollo_auth_state -d "<curl arg -b cookie-file>"
    GET auth_state $argv
end

function apollo_submit_solution -d "<username> <puzzle_id> <solution> [curl args]"
    POST submit -d "{\"username\":\"$argv[1]\",\"puzzle_id\":\"$argv[2]\",\"solution\":\"$argv[3]\"}" $argv[4..]
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
        # return 1
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

function apollo_mock_solutions -d "<from> <to> <password>; solutions: (id*10)"
    for id in (seq $argv[1] $argv[2])
        apollo_set_solution $id (math "$id * 10") $argv[3] &
    end
end

function apollo_mock_join -d "<team-num> <puzzle-num>"
    petname --count $argv[1] | while read -l pn
        apollo_join $pn
        apollo_mock_solve $argv[2] $pn &
    end
end

function apollo_mock_solve -d "<repeat-num> <name>"
    for id in (seq 0 $argv[1])
        set solution (math "$id * 10 + $(random) % 2")
        echo "id: $id => solution: $solution"
        apollo_submit_solution $argv[2] $id $solution
    end
end

function apollo_set_admin_password -d "<password>"
    if test (count $argv) -lt 1
        echo "password not provided"
        return 1
    end
    POST set_admin_password -d "{\"password\":\"$argv[1]\"}"
end
