import MenuElement from "./MenuElement";
import SmallProfileComponent, { Profile } from "./SmallProfile";
import { BiRegularHomeAlt2 } from 'solid-icons/bi'
import { AiOutlineDollar } from 'solid-icons/ai'
import { RiUserGroupLine } from 'solid-icons/ri'
import { FaRegularFaceSmileBeam } from 'solid-icons/fa'
import { BiRegularChat } from 'solid-icons/bi'

export default function SideBar() {
    return(
        <div
            class="w-44 pt-6 flex flex-col
            bg-gray-900 text-white shadow-lg"
        >
            <SmallProfileComponent profile={new Profile("User123")}></SmallProfileComponent>
            <MenuElement text="Home"><BiRegularHomeAlt2 class="menu-icon" size={22} /></MenuElement>
            <MenuElement text="Expenses"><AiOutlineDollar class="menu-icon" size={22} /></MenuElement>
            <MenuElement text="Groups"><RiUserGroupLine class="menu-icon" size={22} /></MenuElement>
            <MenuElement text="Friends"><FaRegularFaceSmileBeam class="menu-icon" size={22} /></MenuElement>
            <MenuElement text="Chats"><BiRegularChat class="menu-icon" size={22} /></MenuElement>
        </div>
    );
}