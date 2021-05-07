#!/usr/bin/env python

import json
import asyncio
from websockets import serve


async def echo(websocket, path):
    async for message in websocket:
        print(f"--> {message}")
        json_msg = json.loads(message);
        if json_msg["method"] == "echo":
            response = {"jsonrpc": "2.0", "id": json_msg["id"], "result": json_msg['params']}
            print("<-- " + json.dumps(response))
            await websocket.send(json.dumps(response))
        if json_msg["method"] == "getCurrentState":
            # Liefert den aktuellen Zustand der Sensoren zurück
            response = {"jsonrpc": "2.0", "id": json_msg["id"], "result": {"temperature": 23.1, "humidity": 50.2, "lightlevel": 80, "co2": 10}}
            print("<-- " + json.dumps(response))
            await websocket.send(json.dumps(response))

async def main():
    print("Starting the server")
    async with serve(echo, "localhost", 8765):
        await asyncio.Future()  # run forever

asyncio.run(main())