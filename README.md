# March Mammal Madness Bracket Backend

This is just a Minimum Viable Product demo, we don't have to use any of this code for the final thing

It provides login functionality with cookies for session management, it will serve stored brackets, and it will score/rank participants. It'll take input primarily as POST raw data, and give output as JSON

## Commands to test out functionality

curl 'http://127.0.0.1:3000/login' -v -X POST --data-raw 'username=ferrrris&password=hunter42&next=%2F'

curl 'http://127.0.0.1:3000/signup' -v -X POST --data-raw 'username=ferrrris&password=hunter42&next=%2F'

curl 'https://127.0.0.1:3000/'
