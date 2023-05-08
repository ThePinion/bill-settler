import { A } from '@solidjs/router'
import { AiOutlinePlus } from 'solid-icons/ai'

export default function AddGroupCard() {
    return(
        <A
            href='/new-group'
            class="flex items-center justify-center rounded-xl px-12 py-8 shadow-md
            bg-gray-700 hover:bg-[#253041] transition-all duration-200 hover:rounded-lg
            text-slate-100 hover:text-slate-300 h-24 opacity-40"
        >
            <AiOutlinePlus size={50}/>
        </A>
    )
}