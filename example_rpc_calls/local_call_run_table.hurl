POST http://0.0.0.0:3030
Content-Type: application/json
{
    "jsonrpc": "2.0",
    "method": "call_run",
    "params": [{"command":{"Table":{"model":"./data/1l_conv/network.onnx"}},"args":{"tolerance":0,"scale":7,"bits":16,"logrows":17,"public_inputs":false,"public_outputs":true,"public_params":false,"max_rotations":512}}],
    "id": 1
}
