POST http://0.0.0.0:3030
Content-Type: application/json
timeout: 1000

{
    "jsonrpc": "2.0",
    "method": "call_run",
    "params": [{"command":{"Prove":{"data":"./data/1l_relu/input.json","model":"./data/network.onnx","vk_path":"1l_relu.vk","proof_path":"1l_relu.pf","params_path":"kzg.params","pfsys":"KZG","transcript":"EVM","strategy":"Single"}},"args":{"tolerance":0,"scale":7,"bits":16,"logrows":17,"public_inputs":false,"public_outputs":true,"public_params":false,"max_rotations":512}}],
    "id": 1
}

