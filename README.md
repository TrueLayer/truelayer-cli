# truelayer-cli


## Use

### Generate webhooks

*Generate executed and settled webhooks*

```
truelayer generate-webhook 
    --private-key {path_to_private_key_pem} 
    --client-id {client_id} 
    --client-secret {client_secret}
    --kid {private_id_key_from_console}
      executed-settled
```

*Generate failed webhook*

```
truelayer generate-webhook 
    --private-key {path_to_private_key_pem} 
    --client-id {client_id} 
    --client-secret {client_secret}
    --kid {private_id_key_from_console}
      failed
```

### Create a tunnel to a local app

Creates an HTTP tunnel to `localhost:8080`

```
truelayer route-webhooks 
    --to-addr http://localhost:8080 
    --client-id {client_id} 
    --client-secret {client_secret}
```
