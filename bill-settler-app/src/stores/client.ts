const url = "ws://localhost:80/rpc";

import { createSignal } from "solid-js";
import { RawClient } from "../lib/client";
import { WebsocketTransport, WebSocketErrorEvent, Request } from "yerpc";
const transport = new WebsocketTransport(url);

const client = new RawClient(transport);
export default client;
