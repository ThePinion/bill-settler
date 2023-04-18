import { A } from "@solidjs/router";
import { Component, ParentComponent } from "solid-js";

const MenuElement: ParentComponent<{text: string}> = (props) => {
    let route = '/' + props.text.toLowerCase();
    return(
        <div 
            class="menu-element"
        >
            <A class="w-full" href={route}>
                <div class="px-4 py-2 flex flex-row items-center gap-3">
                    {props.children}
                    {props.text}
                </div>
            </A>
        </div>
    );
}

export default MenuElement