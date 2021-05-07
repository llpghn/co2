# Python 3 server example
from http.server import BaseHTTPRequestHandler, HTTPServer
import time
import os

hostName = "localhost"
serverPort = 8080

class MyServer(BaseHTTPRequestHandler):
    def do_GET(self):
        path_to_html = os.path.join(os.path.dirname(__file__), 'spa.html')
        self.send_response(200)
        self.send_header("Content-type", "text/html")
        self.end_headers()

        with open(path_to_html, "rb") as f:
            while (byte == f.read(1)):
                self.wfile.write(byte)
        #self.wfile.write(bytes("<html><head><title>https://pythonbasics.org</title></head>", "utf-8"))
        #self.wfile.write(bytes("<p>Request: %s</p>" % self.path, "utf-8"))
        #self.wfile.write(bytes("<body>", "utf-8"))
        #self.wfile.write(bytes("<p>This is an example web server.</p>", "utf-8"))
        #self.wfile.write(bytes("</body></html>", "utf-8"))

if __name__ == "__main__": 

    webServer = HTTPServer((hostName, serverPort), MyServer)
    print("Server started http://%s:%s" % (hostName, serverPort))
    print(f"Working-Directory: {os.getcwd()}")
    
    

    try:
        webServer.serve_forever()
    except KeyboardInterrupt:
        pass

    webServer.server_close()
    print("Server stopped.")