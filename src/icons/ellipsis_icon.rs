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

#[tracing::instrument(level = "debug", skip())]
#[component]
pub fn EllipsisIcon() -> impl IntoView {
    view! {
        <svg
            width="20"
            height="4"
            viewBox="0 0 20 4"
            fill="none"
            style="cursor: pointer;"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M2.38498e-08 2C1.75244e-08 2.53043 0.210714 3.03914 0.585786 3.41421C0.960859 3.78929 1.46957 4 2 4C2.53043 4 3.03914 3.78929 3.41421 3.41421C3.78929 3.03914 4 2.53043 4 2C4 1.46957 3.78929 0.96086 3.41421 0.585787C3.03914 0.210714 2.53043 3.01751e-08 2 2.38498e-08C1.46957 1.75244e-08 0.960859 0.210714 0.585786 0.585787C0.210714 0.96086 3.01751e-08 1.46957 2.38498e-08 2V2ZM8 2C8 2.53043 8.21071 3.03914 8.58579 3.41421C8.96086 3.78929 9.46957 4 10 4C10.5304 4 11.0391 3.78929 11.4142 3.41421C11.7893 3.03914 12 2.53043 12 2C12 1.46957 11.7893 0.96086 11.4142 0.585787C11.0391 0.210714 10.5304 1.25574e-07 10 1.19249e-07C9.46957 1.12923e-07 8.96086 0.210714 8.58579 0.585787C8.21071 0.96086 8 1.46957 8 2V2ZM16 2C16 2.53043 16.2107 3.03914 16.5858 3.41421C16.9609 3.78929 17.4696 4 18 4C18.5304 4 19.0391 3.78929 19.4142 3.41421C19.7893 3.03914 20 2.53043 20 2C20 1.46957 19.7893 0.96086 19.4142 0.585787C19.0391 0.210714 18.5304 2.20973e-07 18 2.14648e-07C17.4696 2.08322e-07 16.9609 0.210714 16.5858 0.585787C16.2107 0.96086 16 1.46957 16 2V2Z"
                fill="var(--textPrimary)"
            ></path>
        </svg>
    }
}
