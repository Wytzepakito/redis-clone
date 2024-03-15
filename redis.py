from redis import Connection

client = Connection(host="localhost", port="6379", db=0)
client.connect()

echo_result = client.echo("Hey")

print(echo_result)