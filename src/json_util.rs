use promkit::jsonstream::JsonStream;
use promkit::serde_json;
use promkit::serde_json::Deserializer;

pub fn json_string() -> JsonStream {
    JsonStream::new(
        Deserializer::from_str(
        r###"{
  "tx": {
    "tx": {
      "source_account": "GAVWTRWEGNCVSFCXGI7ZSWJWL5OKGXPG7ZERDLYP3YNY2WGAF24V6FJ6",
      "fee": 10000,
      "seq_num": 3520722131484673,
      "cond": "none",
      "memo": "none",
      "operations": [
      {
        "source_account": "GAVWTRWEGNCVSFCXGI7ZSWJWL5OKGXPG7ZERDLYP3YNY2WGAF24V6FJ6",
        "body": {
        "invoke_host_function": {
          "host_function": {
            "create_contract_v2": {
              "contract_id_preimage": {
                "address": {
                  "address": "GAVWTRWEGNCVSFCXGI7ZSWJWL5OKGXPG7ZERDLYP3YNY2WGAF24V6FJ6",
                  "salt": "e03cb3a5a72e8d5df0f8783d049a98d59346f92fd3647cb94069ad9d247c4d4c"
                }
              },
              "executable": {
                "wasm": "619f279155fc4e6aa1a79d0782f4bd150c4ef76b3ff0eebd13de6423550184cc"
              },
              "constructor_args": []
            }
          },
          "auth": []
        }
      }
      }
      ],
      "ext": "v0"
    },
    "signatures": [
    {
      "hint": "c02eb95f",
      "signature": "9471e8447513832b962cdea793360bf06ab03691b31b7a18d66ab39a892538ac826d269dca89492cea4d3cda91f1c7a453b900b635bfe51986dc3c928c893f03"
    }
    ]
  }
}"###, ).into_iter::<serde_json::Value>()
             .filter_map(serde_json::Result::ok)
             .collect::<Vec<_>>()
             .iter())
}
