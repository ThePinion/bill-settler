import { A, useParams } from "@solidjs/router";
import { Show, createResource, createSignal } from "solid-js";
import ExpenseComponent from "../common/Expense";

const fetchExpense = async (id: number) => {
    const res = await fetch('http://localhost:4000/expenses/' + id)

    return res.json()
}

export default function DetailedExpense() {
    const params = useParams()
    const [expense] = createResource(+(params.id), fetchExpense)
    const [editMode, setEditMode] = createSignal(false)
    const [editString, setEditString] = createSignal('Edit')

    const changeEditMode = () => {
        setEditMode(!editMode());
        if(editMode()) setEditString("Save")
        else setEditString("Edit")
    }

    return(
        <div 
            class="bg-gray-800 text-white h-screen flex justify-center items-center"
        >
            <div
                class="p-8 rounded-lg shadow-xl bg-[#253041]"
            >
                <Show when={expense()}>
                    <div class="grid grid-cols-2 gap-x-16 gap-y-8">
                        <div>
                            <label class="block font-bold mb-2 " for="name">Name</label>
                            <input
                                class="block w-full px-3 py-2 rounded-lg bg-gray-700 shadow-md border border-gray-700 focus:outline-none focus:border-blue-500"
                                type="text"
                                id="name"
                                name="name"
                                disabled={!editMode()}
                                value={expense().name}
                            />
                        </div>
                        <div>
                            <label class="block font-bold mb-2 " for="owner">Owner</label>
                            <input
                                class="block w-full px-3 py-2 rounded-lg bg-gray-700 shadow-md border border-gray-700 focus:outline-none focus:border-blue-500"
                                type="text"
                                id="owner"
                                name="owner"
                                disabled={!editMode()}
                                value={expense().owner}
                            />
                        </div>
                        <div>
                            <label class="block font-bold mb-2 " for="cost">Cost</label>
                            <input
                                class="block w-full px-3 py-2 rounded-lg bg-gray-700 shadow-md border border-gray-700 focus:outline-none focus:border-blue-500"
                                type="number"
                                id="cost"
                                name="cost"
                                disabled={!editMode()}
                                value={expense().cost}
                            />
                        </div>
                        <div>
                            <label class="block font-bold mb-2 " for="status">Settled</label>
                            <input
                                class="block w-full px-3 py-2 rounded-lg bg-gray-700 shadow-md border border-gray-700 focus:outline-none focus:border-blue-500"
                                type="text"
                                id="status"
                                name="status"
                                disabled={!editMode()}
                                value={expense().settled}
                            />
                        </div>
                    </div>
                    <label class="block font-bold mb-2 mt-8" for="paid_for">Paid For</label>
                    <input
                        class="block w-full px-3 py-2 rounded-lg bg-gray-700 shadow-md border border-gray-700 focus:outline-none focus:border-blue-500"
                        type="text"
                        id="paid_for"
                        name="paid_for"
                        disabled={!editMode()}
                    />
                    <div class="flex flex-row gap-8 items-center justify-center mt-8">
                        <button 
                            class="main-button bg-teal-600 hover:bg-teal-700"
                            onClick={changeEditMode}
                        >
                            {editString()}
                        </button>
                        <A href="/home">
                            <button class="main-button bg-yellow-600 hover:bg-yellow-700">
                                Exit
                            </button>
                        </A>
                    </div>
                </Show>
            </div>
            
        </div>
    )
}