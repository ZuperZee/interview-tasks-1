# Tasks

## Client/Server

Create a server that reads from a JSON file.
Example file:

```json
{
  "servers": [
    {
      "socket_address": "127.0.0.1:3000",
      "response": "5 cats"
    },
    {
      "socket_address": "127.0.0.1:3001",
      "response": "16 chickens"
    },
    {
      "socket_address": "127.0.0.1:3002",
      "response": [51, 32, 98, 117, 110, 110, 105, 101, 115]
    }
  ]
}
```

**Server**
Open a TCP listener for each server on the specified socket address from the JSON file.
Respond with the response, which can either be a string or an array of numbers (bytes).

Following the example file above:
3 TCP listeners should start.

- The first listener on socket address 127.0.0.1:3000 should respond with "5 cats".
- The second listener on socket address 127.0.0.1:3001 should respond with "16 chickens".
- The third listener on socket address 127.0.0.1:3002 should respond with [51, 32, 98, 117, 110, 110, 105, 101, 115] ("3 bunnies").

**Client**
Read the data returned from the server on the specified socket address.
The socket address can be provided as a command line argument.
The read data should be parsed into a JSON object with the byte index as the key and the byte as the value.
If the byte is an ASCII digit, write it as a number. The rest of the bytes should be written as strings.
Print the JSON object.

Following the example file above:

- If a client starts with socket address 127.0.0.1:3000 as the command line argument. The response should be "5 cats" and the printed value should be

  ```json
  {
    "0": 5,
    "1": " ",
    "2": "c",
    "3": "a",
    "4": "t",
    "5": "s"
  }
  ```

- If a client starts with socket address 127.0.0.1:3001 as the command line argument. The response should be "16 chickens" and the printed value should be

  ```json
  {
    "0": 1,
    "1": 6,
    "2": " ",
    "3": "c",
    "4": "h",
    "5": "i",
    "6": "c",
    "7": "k",
    "8": "e",
    "9": "n",
    "10": "s"
  }
  ```

- If a client starts with socket address 127.0.0.1:3002 as the command line argument. The response should be "3 bunnies" and the printed value should be

  ```json
  {
    "0": 3,
    "1": " ",
    "2": "b",
    "3": "u",
    "4": "n",
    "5": "n",
    "6": "i",
    "7": "e",
    "8": "s"
  }
  ```

## Converter

Create a converter that converts the example input below into the example output below.

Example:

`input`

```json
{
  "servers": [
    {
      "socket_address": "127.0.0.1:3000",
      "response": "5 cats"
    },
    {
      "socket_address": "127.0.0.1:3001",
      "response": "16 chickens"
    },
    {
      "socket_address": "127.0.0.1:3002",
      "response": [51, 32, 98, 117, 110, 110, 105, 101, 115]
    }
  ]
}
```

`output`

```json
{
  "servers": [
    {
      "socket_address": "127.0.0.1:3000",
      "response": {
        "0": 5,
        "1": " ",
        "2": "c",
        "3": "a",
        "4": "t",
        "5": "s"
      }
    },
    {
      "socket_address": "127.0.0.1:3001",
      "response": {
        "0": 1,
        "1": 6,
        "2": " ",
        "3": "c",
        "4": "h",
        "5": "i",
        "6": "c",
        "7": "k",
        "8": "e",
        "9": "n",
        "10": "s"
      }
    },
    {
      "socket_address": "127.0.0.1:3002",
      "response": {
        "0": 3,
        "1": " ",
        "2": "b",
        "3": "u",
        "4": "n",
        "5": "n",
        "6": "i",
        "7": "e",
        "8": "s"
      }
    }
  ]
}
```
