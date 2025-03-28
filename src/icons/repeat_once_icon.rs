// Moosync
// Copyright (C) 2024, 2025  Moosync <support@moosync.app>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use leptos::{component, prelude::*, view, IntoView};

#[tracing::instrument(level = "debug", skip(_filled))]
#[component]
pub fn RepeatOnceIcon(#[prop()] _filled: ReadSignal<bool>) -> impl IntoView {
    view! {
        <svg
            width="27"
            height="27"
            viewBox="0 0 27 27"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M26.0861 12.0223C25.7672 10.3767 24.9825 8.85737 23.8251 7.64494H23.8273L23.6698 7.48744C23.4576 7.29866 23.1803 7.19982 22.8966 7.21181C22.6128 7.2238 22.3449 7.34569 22.1494 7.5517C21.9539 7.7577 21.8462 8.03167 21.8491 8.31566C21.8519 8.59964 21.9652 8.87137 22.1648 9.07338C23.3808 10.3006 24.0618 11.9592 24.0592 13.6868L24.0482 14.0718C23.9495 15.7424 23.2162 17.3122 21.9984 18.4601C20.7806 19.608 19.1702 20.2473 17.4967 20.2471H10.2932H7.20009L6.87853 20.5687L6.75384 20.7196C6.60798 20.9301 6.54057 21.185 6.56336 21.44C6.58614 21.6951 6.69768 21.934 6.87853 22.1153L11.2579 26.4946L11.4088 26.6215C11.6195 26.7671 11.8746 26.8341 12.1296 26.8109C12.3847 26.7878 12.6235 26.6758 12.8045 26.4946L12.9313 26.3437C13.0769 26.133 13.144 25.8779 13.1208 25.6229C13.0976 25.3679 12.9857 25.129 12.8045 24.9481L10.2932 22.4346L17.4967 22.4368L17.932 22.4259C19.606 22.3423 21.2209 21.7797 22.5845 20.805C23.948 19.8302 25.0029 18.4843 25.6237 16.9273C26.2444 15.3703 26.4049 13.6678 26.0861 12.0223Z"
                fill=move || if !_filled.get() { "var(--textSecondary)" } else { "var(--accent)" }
            ></path>
            <path
                d="M14.8367 0.752128L14.9898 0.876816L19.367 5.25619L19.4938 5.40932C19.6199 5.59197 19.6874 5.80864 19.6874 6.03057C19.6874 6.25249 19.6199 6.46916 19.4938 6.65182L19.367 6.80494L19.0476 7.12432H15.9523H8.74665C7.07275 7.12422 5.46209 7.76378 4.24421 8.91212C3.02633 10.0605 2.29329 11.6308 2.19509 13.3018L2.18415 13.6868C2.18415 15.4806 2.90384 17.1037 4.06759 18.2893C4.25187 18.4997 4.34854 18.7728 4.33771 19.0523C4.32687 19.3318 4.20937 19.5965 4.00936 19.792C3.80935 19.9876 3.54202 20.099 3.26235 20.1035C2.98268 20.108 2.71191 20.0052 2.50572 19.8162C1.32454 18.6129 0.51619 17.094 0.177908 15.4421C-0.160375 13.7903 -0.0141796 12.0758 0.598906 10.5051C1.21199 8.93438 2.26585 7.57424 3.63371 6.58833C5.00157 5.60241 6.62516 5.03271 8.30915 4.94775L8.74665 4.93682L15.9523 4.93463L13.441 2.42557L13.3163 2.27244C13.1723 2.0619 13.1063 1.80761 13.1299 1.55359C13.1535 1.29956 13.2652 1.06178 13.4456 0.881389C13.626 0.700994 13.8638 0.589333 14.1178 0.565729C14.3718 0.542124 14.6261 0.60806 14.8367 0.752128Z"
                fill=move || if !_filled.get() { "var(--textSecondary)" } else { "var(--accent)" }
            ></path>
            <path
                d="M12.6178 12.0051H11.1178V10.5051L13.1271 10.5051L14.1178 10.5051V11.4982V16.5051H13.0272H12.6178V16.0367V12.0051Z"
                fill=move || if !_filled.get() { "var(--textSecondary)" } else { "var(--accent)" }
            ></path>
        </svg>
    }
}
