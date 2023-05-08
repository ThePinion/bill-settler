import { A } from "@solidjs/router";


export default function NewGroup() {
    return(
        <div 
            class="bg-gray-800 text-white h-screen flex justify-center items-center"
        >
            <div
                class="p-8 rounded-lg shadow-xl bg-[#253041]"
            >
                <h1 class="font-bold text-2xl mb-6 flex justify-center">
                    New Group
                </h1>
                <form>
                    <label class="block font-bold mb-2 " for="expense_name">Name</label>
                    <input
                        class="block w-full px-3 py-2 rounded-lg bg-gray-700 shadow-md border border-gray-700 focus:outline-none focus:border-blue-500"
                        type="text"
                        id="expense_name"
                        name="expense_name"
                    />
                    <label class="block font-bold my-2" for="members">Members</label>
                    <input
                        class="block w-full px-3 py-2 rounded-lg bg-gray-700 shadow-md border border-gray-700 focus:outline-none focus:border-blue-500"
                        type="text"
                        id="members"
                        name="members"
                    />
                    <div class="flex flex-row gap-8 items-center justify-center mt-8">
                        <A href="/groups">
                            <button class="main-button bg-teal-600 hover:bg-teal-700">
                                Create Group
                            </button>
                        </A>
                        <A href="/groups">
                            <button class="main-button bg-yellow-600 hover:bg-yellow-700">
                                Cancel
                            </button>
                        </A>
                    </div>
                </form>
            </div>
            
        </div>
    )
}