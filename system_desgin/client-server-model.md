STEPS:
- When you make a request to a domain, 
- The client checks the DNS for the IP of the domain which is provided by the server the app is hosted on.
- Returns the IP address of the domain
- The client then makes HTTP request to the IP at the port to the server.
- Server responds to the request


- Check IP address of a domain
  ```
  dig codebreather.com
  ```

- Communicate on your Unix machine
  Open two terminals
  On first enter 
  `nc -1 8081`

  On second:
  `nc 127.0.0.1 8081`

Entering any word on the second terminal shows on the first one listening to it on the port.
