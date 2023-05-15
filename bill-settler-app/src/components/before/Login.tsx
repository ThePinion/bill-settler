import { A, useNavigate } from "@solidjs/router";
import client from "../../stores/client";

type LoginRequest = {
  handle: string;
  password: string;
};

export default function Login() {
  const navigate = useNavigate()
  const handleSubmit = async (event: Event) => {
    event.preventDefault();
    const formData = new FormData(event.target as HTMLFormElement);
    const loginRequest: LoginRequest = {
      handle: formData.get("handle") as string,
      password: formData.get("password") as string,
    };
    let res = await client.login(loginRequest)
    console.log(res)
    if(res != null)navigate("/home")
  };

  return (
    <div class="max-w-md mx-auto">
      <div class="p-8 rounded-lg shadow-xl bg-gray-800">
        <h1 class="text-center text-3xl font-bold mb-6 text-blue-400">
          Login
        </h1>
        <form onSubmit={handleSubmit}>
          <div class="mb-4">
            <label class="block font-bold mb-2" for="handle">
              Handle
            </label>
            <input
              class="block w-full px-3 py-2 rounded-lg bg-gray-700 shadow-md border border-gray-700 focus:outline-none focus:border-blue-500"
              type="text"
              id="handle"
              name="handle"
            />
          </div>
          <div class="mb-4">
            <label class="block font-bold mb-2" for="password">
              Password
            </label>
            <input
              class="block w-full px-3 py-2 rounded-lg bg-gray-700 shadow-md border border-gray-700 focus:outline-none focus:border-blue-500"
              type="password"
              id="password"
              name="password"
            />
          </div>
          <button
            class="text-center block w-full px-4 py-2 mt-4 shadow-md font-bold text-white bg-blue-500 rounded-lg hover:bg-gray-600"
            type="submit"

          >
            Log In
          </button>
        </form>
      </div>
      <div class="my-5 p-8 rounded-lg shadow-xl bg-gray-800">
        <p class="text-center">Don't have an account?</p>
        <A
          class="text-center block w-full px-4 py-2 mt-2 font-bold shadow-md text-white bg-gray-700 rounded-lg hover:bg-gray-600"
          href="/before/register"
        >
          Register
        </A>
      </div>
    </div>
  );
}
