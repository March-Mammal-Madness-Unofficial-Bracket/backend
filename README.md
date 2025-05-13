# March Mammal Madness Bracket Backend

It provides login functionality with cookies for session management, it will serve stored brackets, and it will score/rank participants. It'll take input primarily as POST raw data, and give output as JSON

## Commands to test out functionality

curl 'http://127.0.0.1:3000/login' -v -X POST --data-raw 'username=ferrrris&password=hunter42&next=%2F'

curl 'http://127.0.0.1:3000/signup' -v -X POST --data-raw 'username=ferrrris&password=hunter42&next=%2F'
