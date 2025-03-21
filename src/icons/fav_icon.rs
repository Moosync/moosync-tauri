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

#[tracing::instrument(level = "debug", skip(filled))]
#[component]
pub fn FavIcon<T>(#[prop()] filled: T) -> impl IntoView
where
    T: Get<Value = bool> + 'static + Send,
{
    view! {
        <svg
            class="button-grow"
            width="26"
            height="22"
            viewBox="0 0 26 22"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M11.8865 4.18463L12.5416 5.35955L13.1967 4.18465C13.5808 3.4957 14.2218 2.57151 15.165 1.8556L14.7192 1.2683L15.165 1.8556C16.1365 1.11817 17.2257 0.75 18.4205 0.75C21.7611 0.75 24.3332 3.41854 24.3332 7.09272C24.3332 9.04171 23.5466 10.7155 22.0473 12.4911C20.5289 14.2892 18.3414 16.122 15.6193 18.3988L16.0716 18.9396L15.6193 18.3988L15.6187 18.3992C14.7 19.1676 13.6554 20.0413 12.5691 20.9737L12.5688 20.974C12.5638 20.9783 12.5544 20.9829 12.5416 20.9829C12.5288 20.9829 12.5195 20.9783 12.5145 20.974L12.5141 20.9737C11.4285 20.0419 10.3845 19.1687 9.46669 18.4011L9.46455 18.3993L9.46454 18.3993C6.74207 16.1223 4.55442 14.2894 3.03603 12.4912C1.53659 10.7155 0.75 9.04171 0.75 7.09272C0.75 3.41854 3.32209 0.75 6.66273 0.75C7.85754 0.75 8.94668 1.11817 9.91822 1.8556L10.3717 1.2582L9.91822 1.8556C10.8614 2.57151 11.5024 3.49566 11.8865 4.18463Z"
                fill=move || if !filled.get() { "transparent" } else { "var(--accent)" }
                stroke="var(--accent)"
                stroke-width="1.5"
            ></path>
        </svg>
    }
}
