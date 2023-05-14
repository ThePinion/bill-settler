import { Suspense, createResource, lazy } from "solid-js"
import client from "../../stores/client"


export default function Playground() {
    let fetchMsg = async () => await client.helloWorld()
    const [msg] = createResource(fetchMsg)
    return <div
      class="bg-gray-900 text-white min-h-screen flex justify-center items-center"
    >
        {msg()}
      
    </div>
  }