It's a common concept to put a proxy in front of S3 that handles requests. Nginx is well suited for this and can be used to additionally handle TLS and path style (using subdomains instead of subfolders).

### Example Nginx config

```
upstream seaweedfs { server localhost:8333 fail_timeout=0; }

server {
	listen 443 ssl;
	server_name ~^(?<subdomain>[^.]+).yourdomain.com;

	ignore_invalid_headers off;
	client_max_body_size 0;
	proxy_buffering off;

	location / {
		proxy_set_header Host $host;
		proxy_set_header X-Real-IP $remote_addr;
		proxy_set_header X-Forwarded-For $remote_addr;
		proxy_set_header X-Forwarded-Proto $scheme;

		proxy_connect_timeout 300;
		proxy_http_version 1.1;
		proxy_set_header Connection "";
		chunked_transfer_encoding off;

		proxy_pass http://seaweedfs$request_uri;
	}

	ssl on;
	ssl_certificate /{path_to_ssl_cert}/cert.pem;
	ssl_certificate_key /{path_to_ssl_cert}/key.pem;
}