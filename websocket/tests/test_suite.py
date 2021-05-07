#!/usr/bin/env python

# WS client example

import asyncio
import websockets

testCases = [
  {
    "id": 'Test echo function... ',
    "send": '{"jsonrpc": "2.0", "method": "echo", "params": "Ich bin ein String", "id": 1}',
    "expect": '{"jsonrpc": "2.0", "id": 1, "result": "Ich bin ein String"}'
  }

]

async def hello():
    uri = "ws://localhost:8765"
    async with websockets.connect(uri) as websocket:
        # "jsonrpc": "2.0", "method": "subtract", "params": [42, 23], "id": 1
        for i in testCases:
            print(f"{i['id']}", end='')
            await websocket.send(i["send"])
            res = await websocket.recv()
            if res != i['expect']:
              print("ERROR")
              print(f"\t\t<-- {res}")
            else:
              print("OK")
        
        
asyncio.get_event_loop().run_until_complete(hello())