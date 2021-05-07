echo Starting Websocket-Server
python3 ../src/main.py &
PID_OF_SERVER=$!
echo PID of server: $PID_OF_SERVER
echo Start testing!
python3 test_suite.py 


echo tests endet, killing server process
kill $PID_OF_SERVER