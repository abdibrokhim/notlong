some testing sutff:

[shorten]

[shorten_url]:
for testing the shorten endpoint, you can use the following curl command:
curl -X POST -H "Content-Type: application/json" \
     -d '{"original_url":"https://www.linkedin.com/posts/abdibrokhim_i-know-you-are-struggling-to-find-an-idea-activity-7285736275242864644-ehpY?utm_source=share&utm_medium=member_desktop"}' \
     http://localhost:8000/shorten

then open browser and go to http://localhost:8000/replace_with_your_short_code to see the redirect in action.

[response]:
{"id":1,"original_url":"https://www.linkedin.com/posts/abdibrokhim_i-know-you-are-struggling-to-find-an-idea-activity-7285736275242864644-ehpY?utm_source=share&utm_medium=member_desktop","short_code":"TTMCeh","created_at":"2025-01-16T21:13:55.281419"}


[wallet]

[create_wallet]:
for testing the wallet creation endpoint, you can use the following curl command:
curl -X POST -H "Content-Type: application/json" \
     -d '{"wallet_address":"6Wv8kxRJsNxmh2e2wdpXNnLpFQuik59Gghk2nDvSCaHL"}' \
     http://localhost:8000/wcreate

[response]:
{"id":1,"wallet_address":"6Wv8kxRJsNxmh2e2wdpXNnLpFQuik59Gghk2nDvSCaHL","tries_left":5,"created_at":"2025-01-17T00:53:44.988324"}

my sol address: 6Wv8kxRJsNxmh2e2wdpXNnLpFQuik59Gghk2nDvSCaHL . feel free to send some sol to me. LOL

[update_wallet_tries]:
curl -X POST -H "Content-Type: application/json" \
     -d '{"wallet_address":"6Wv8kxRJsNxmh2e2wdpXNnLpFQuik59Gghk2nDvSCaHL"}' \
     http://localhost:8000/wupdate

[response]:
{"id":1,"wallet_address":"6Wv8kxRJsNxmh2e2wdpXNnLpFQuik59Gghk2nDvSCaHL","tries_left":4,"created_at":"2025-01-17T00:53:44.988324"}