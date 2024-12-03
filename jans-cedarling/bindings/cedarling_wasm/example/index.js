import { Cedarling } from "cedarling_wasm";
import { policy_store } from "./policy_store";

let cedarling = Cedarling.new({
  "CEDARLING_APPLICATION_NAME": "TestApp",
  "POLICY_STORE_ID": "asdasd123123",
  "CEDARLING_LOCAL_POLICY_STORE": policy_store,
});

// cedarling.authorize();
