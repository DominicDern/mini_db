use std::collections::HashSet;

use iced::widget::{button, column, container, mouse_area, row, scrollable, text, text_input};
use iced::{Element, Length};

use crate::db::container::{ContainerNode, find_path};
use crate::db::id::Id;
use crate::ui::messages::{DBMessage, Message, UIMessage};
use crate::ui::state::App;

pub fn view(state: &App) -> Element<'_, Message> {
    let tree_items: Vec<Element<'_, Message>> = state
        .containers
        .iter()
        .map(|node| {
            render_node(
                node,
                &state.container_state.expanded,
                state.container_state.selected,
                state.container_state.context_menu_open_for,
                &state.container_state.add_container_state.name_input,
                0,
            )
        })
        .collect();

    let tree: Element<'_, Message> = if tree_items.is_empty() {
        text("No containers yet — add a root container to get started.").into()
    } else {
        column(tree_items).spacing(2).into()
    };

    let sidebar = container(
        column![
            row![
                text("Containers").size(20),
                button("Home").on_press(Message::UI(UIMessage::NavigateHome)),
            ]
            .spacing(10),
            scrollable(tree).height(Length::Fill),
        ]
        .spacing(10),
    )
    .width(Length::FillPortion(1))
    .padding(10);

    let detail = container(detail_panel(state))
        .width(Length::FillPortion(2))
        .padding(10);

    row![sidebar, detail].spacing(20).into()
}

/// Recursively renders one node of the forest, plus its children if expanded.
/// A chevron button toggles expansion; clicking the name selects the
/// container (loading its detail in the right-hand panel); right-clicking
/// the row opens an inline context menu with add/remove actions.
fn render_node<'a>(
    node: &'a ContainerNode,
    expanded: &HashSet<Id>,
    selected: Option<Id>,
    context_menu_open_for: Option<Id>,
    name_input: &'a str,
    depth: u16,
) -> Element<'a, Message> {
    let id = node.container.id;
    let is_expanded = expanded.contains(&id);
    let has_children = !node.children.is_empty();

    let chevron: Element<'_, Message> = if has_children {
        button(text(if is_expanded { "▼" } else { "▶" }))
            .on_press(Message::UI(UIMessage::ContainerExpandToggled(id)))
            .into()
    } else {
        text("  ").into()
    };

    let label = if selected == Some(id) {
        format!("[{}]", node.container.name)
    } else {
        node.container.name.clone()
    };

    let name_button = button(text(label)).on_press(Message::UI(UIMessage::ContainerSelected(id)));

    let indent = iced::Padding {
        top: 0.0,
        right: 0.0,
        bottom: 0.0,
        left: depth as f32 * 16.0,
    };

    // Right-clicking anywhere on the row toggles this node's context menu.
    let row_content = mouse_area(row![chevron, name_button].spacing(4))
        .on_right_press(Message::UI(UIMessage::ContainerContextMenuToggled(id)));

    let this_row = container(row_content).padding(indent);

    let mut items: Vec<Element<'_, Message>> = vec![this_row.into()];

    if context_menu_open_for == Some(id) {
        items.push(context_menu(id, name_input, depth));
    }

    if is_expanded {
        for child in &node.children {
            items.push(render_node(
                child,
                expanded,
                selected,
                context_menu_open_for,
                name_input,
                depth + 1,
            ));
        }
    }

    column(items).spacing(2).into()
}

/// The inline right-click menu for a tree node: add a child here, add a new
/// root container, or remove this node. Add actions reuse whatever name is
/// currently typed into the "add container" form.
fn context_menu<'a>(id: Id, name_input: &'a str, depth: u16) -> Element<'a, Message> {
    let name_is_empty = name_input.trim().is_empty();

    let add_here: Element<'_, Message> = if name_is_empty {
        button("Add here").into()
    } else {
        button("Add here")
            .on_press(Message::DB(DBMessage::AddContainer(
                name_input.to_string(),
                Some(id),
            )))
            .into()
    };

    let add_root: Element<'_, Message> = if name_is_empty {
        button("Add root container").into()
    } else {
        button("Add root container")
            .on_press(Message::DB(DBMessage::AddContainer(
                name_input.to_string(),
                None,
            )))
            .into()
    };

    let remove = button("Remove")
        .on_press(Message::DB(DBMessage::RemoveContainer(id)));

    let indent = iced::Padding {
        top: 0.0,
        right: 0.0,
        bottom: 0.0,
        left: depth as f32 * 16.0 + 20.0,
    };

    container(
        column![
            text("Type a name in the form below, then:"),
            row![add_here, add_root, remove].spacing(8),
        ]
        .spacing(4),
    )
    .padding(indent)
    .into()
}

fn detail_panel(state: &App) -> Element<'_, Message> {
    let Some(selected_id) = state.container_state.selected else {
        return column![
            text("Select a container on the left, or add a root container below."),
            add_container_form(state),
        ]
        .spacing(12)
        .into();
    };

    let Some(contents) = &state.container_state.contents else {
        return text("Loading container...").into();
    };

    let breadcrumb = find_path(&state.containers, selected_id)
        .map(|path| {
            path.iter()
                .map(|c| c.name.clone())
                .collect::<Vec<_>>()
                .join(" / ")
        })
        .unwrap_or_else(|| contents.container.name.clone());

    let header = column![
        text(breadcrumb).size(14),
        text(contents.container.name.clone()).size(22),
    ]
    .spacing(2);

    let child_names: Vec<Element<'_, Message>> = contents
        .child_containers
        .iter()
        .map(|c| text(c.name.clone()).into())
        .collect();
    let child_list: Element<'_, Message> = if child_names.is_empty() {
        text("(none)").into()
    } else {
        column(child_names).spacing(4).into()
    };

    let mini_names: Vec<Element<'_, Message>> = contents
        .minis
        .iter()
        .map(|m| text(m.name.clone()).into())
        .collect();
    let mini_list: Element<'_, Message> = if mini_names.is_empty() {
        text("(none)").into()
    } else {
        column(mini_names).spacing(4).into()
    };

    let terrain_names: Vec<Element<'_, Message>> = contents
        .terrain
        .iter()
        .map(|t| text(t.name.clone()).into())
        .collect();
    let terrain_list: Element<'_, Message> = if terrain_names.is_empty() {
        text("(none)").into()
    } else {
        column(terrain_names).spacing(4).into()
    };

    column![
        header,
        text("Child containers:"),
        child_list,
        text("Minis:"),
        mini_list,
        text("Terrain:"),
        terrain_list,
        add_container_form(state),
    ]
    .spacing(10)
    .into()
}

/// Inline form for adding a new container. "Add here" adds as a child of
/// whatever container is currently selected (disabled if nothing is
/// selected); "Add root container" always adds a brand new top-level
/// container regardless of the current selection.
fn add_container_form(state: &App) -> Element<'_, Message> {
    let name = state.container_state.add_container_state.name_input.clone();
    let selected = state.container_state.selected;
    let name_is_empty = name.trim().is_empty();

    let add_here_button: Element<'_, Message> = match selected {
        Some(parent_id) if !name_is_empty => button("Add here")
            .on_press(Message::DB(DBMessage::AddContainer(
                name.clone(),
                Some(parent_id),
            )))
            .into(),
        _ => button("Add here").into(),
    };

    let add_root_button: Element<'_, Message> = if name_is_empty {
        button("Add root container").into()
    } else {
        button("Add root container")
            .on_press(Message::DB(DBMessage::AddContainer(name.clone(), None)))
            .into()
    };

    let hint = match selected {
        Some(_) => "\"Add here\" nests inside the selected container. \"Add root container\" always creates a new top-level container.",
        None => "Nothing selected — \"Add root container\" will create a new top-level container.",
    };

    column![
        text(hint),
        row![
            text_input("New container name", &name)
                .on_input(|value| Message::UI(UIMessage::ContainerNameInputChanged(value))),
            add_here_button,
            add_root_button,
        ]
        .spacing(10),
    ]
    .spacing(6)
    .into()
}
