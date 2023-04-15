import { A } from "@solidjs/router";
import { Component } from "solid-js";

const MenuElement: Component<{text: string}> = (props) => {
    let route = '/' + props.text.toLowerCase();
    return(
        <div 
            class="menu-element"
        >
            <A class="w-full" href={route}>
                <div class="px-4 py-2">
                    {props.text}
                </div>
            </A>
        </div>
    );
}

export default MenuElement