POST http://0.0.0.0:3030
Content-Type: application/json

{
    "jsonrpc": "2.0",
    "method": "mock",
    "params": [
        {
            "command": {
                "Mock": {
                    "data": "./data/1l_relu/input2.json",
                    "model": "./data/1l_relu/network.onnx"
                }
            },
            "args": {
                "tolerance": 0,
                "scale": 4,
                "bits": 10,
                "logrows": 12,
                "public_inputs": false,
                "public_outputs": true,
                "public_params": false,
                "max_rotations": 512
            }
        },
        {
            "input_data": [[-0.40077725, 2.4938452, 0.5796361]],
            "input_shapes": [[3]],
            "output_data": [[0.0, 2.5, 0.5625]]
        }
    ],
    "id": 1
}
