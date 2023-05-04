import { Component } from "solid-js"
import Gap from "../common/Gap";

const Balance: Component<{owe: number, owed: number}> = (props) => {
    return(
        <div
            class="flex flew-wrap flex-row bg-gray-700 rounded-xl p-4 gap-2
            font-semibold shadow-sm"
        >
            <h1>You owe</h1>
            <h1>{props.owe.toFixed(2)}$</h1>
            <Gap/>
            <h1>You are owed</h1>
            <h1>{props.owed.toFixed(2)}$</h1>
            <Gap/>
            <h1>Total</h1>
            <h1>{(props.owed - props.owe).toFixed(2)}$</h1>
        </div>
    );
}

export default Balance