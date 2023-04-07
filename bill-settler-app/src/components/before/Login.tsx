import { A } from "@solidjs/router";

export default function Login() {
  return <div class="max-w-md mx-auto">
    <div class=" p-8 rounded-lg shadow-xl bg-gray-800">
      <h1 class="text-center text-3xl font-bold mb-6 text-blue-400">Login</h1>
      <form>
        <div class="mb-4">
          <label class="block font-bold mb-2 " for="username">Username</label>
          <input
            class="block w-full px-3 py-2 rounded-lg bg-gray-700 shadow-md border border-gray-700 focus:outline-none focus:border-blue-500"
            type="text"
            id="username"
            name="username"
          />
        </div>
        <div class="mb-4">
          <label class="block font-bold mb-2" for="password">Password</label>
          <input
            class="block w-full px-3 py-2 rounded-lg bg-gray-700 shadow-md border border-gray-700 focus:outline-none focus:border-blue-500"
            type="password"
            id="password"
            name="password"
          />
        </div>
        <A
          class="text-center block w-full px-4 py-2 mt-4 shadow-md font-bold text-white bg-blue-500 rounded-lg hover:bg-gray-600"
          type="submit"
          href="/before/login"
        >
          Log In
        </A>
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
}