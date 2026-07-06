import re

with open("crates/trinity/static/phone.html", "r") as f:
    content = f.read()

# Split into CSS, HTML body, and JS
head_end = content.find("</head>")
script_start = content.find("<script>")

head = content[:head_end]
body = content[head_end + 7:script_start]
js = content[script_start:]

# Save them
with open("scratch/phone_body.html", "w") as f: f.write(body)
with open("scratch/phone_js.html", "w") as f: f.write(js)

