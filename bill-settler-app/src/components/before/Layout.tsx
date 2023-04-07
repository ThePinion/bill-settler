import { Outlet, Route } from "@solidjs/router";
import Login from "./Login";

export default function BeforeLayout() {
  return <div
    class="bg-gray-900 text-white min-h-screen flex justify-center items-center"
  >
    <Outlet />
  </div>
}

