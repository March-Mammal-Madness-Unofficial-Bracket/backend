# March Mammal Madness Bracket Backend

This is just a Minimum Viable Product demo, we don't have to use any of this code for the final thing

It provides login functionality with cookies for session management, it will serve stored brackets, and it will score/rank participants. It'll take input primarily as POST raw data, and give output as JSON

## Commands to test out functionality

curl 'http://127.0.0.1:3000/login' -v -X POST --data-raw 'username=ferrrris&password=hunter42&next=%2F'

curl 'http://127.0.0.1:3000/signup' -v -X POST --data-raw 'username=ferrrris&password=hunter42&grade=9&realname=ferris&next=%2F'

curl 'http://127.0.0.1:3000/bracket' -v

curl 'http://127.0.0.1:3000/leaderboard' -v

curl 'http://127.0.0.1:3000/new_bracket' -v -X POST --data-raw 'one=a&two=b&three=c'

curl 'http://127.0.0.1:3000/update_bracket' -v -X POST -H "Content-Type: application/json" --json '{ "Round 1": ["Animal 1", "Animal 2", "Animal 3", "Animal 4"], "Round 2": ["Animal 1", "Animal 2", "Animal 3", "Animal 4"], "Round 3": ["Animal 1", "Animal 2", "Animal 3", "Animal 4"], "Round 4": ["Animal 1", "Animal 2", "Animal 3", "Animal 4"], "Round 5": ["Animal 1", "Animal 2", "Animal 3", "Animal 4"], "Champion": "Animal 8", "Wild Card": "Animal 10" }' 

curl 'http://127.0.0.1:3000/bracket' -v -X POST -H "Content-Type: application/json" --json '{ "Round 1": ["Animal 1", "Animal 2", "Animal 3", "Animal 4"], "Round 2": ["Animal 1", "Animal 2", "Animal 3", "Animal 4"], "Round 3": ["Animal 1", "Animal 2", "Animal 3", "Animal 4"], "Round 4": ["Animal 1", "Animal 2", "Animal 3", "Animal 4"], "Round 5": ["Animal 1", "Animal 2", "Animal 3", "Animal 4"], "Champion": "Animal 8", "Wild Card": "Animal 10" }' 
