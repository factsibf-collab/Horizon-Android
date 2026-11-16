use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use super::*;
use layout::Placement;

//! User-added keys share drawing/hit geometry and retain per-finger ownership.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MacroEvent {
    pub key: String,
    pub delay_ms: u64,
    pub pressed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomButton {
    pub key: String,

    #[serde(default)]
    pub macro_queue: VecDeque<MacroEvent>,

    pub second_key: String,

    #[serde(default)]
    pub third_key: String,

    #[serde(default)]
    pub fourth_key: String,

    #[serde(default)]
    pub custom_label: String,

    #[serde(default)]
    pub always_visible: bool,

    pub layouts: [Placement; 3],
}

impl CustomButton {
    pub fn new(key: String, index: usize) -> Self {
        Self {
            key,
            macro_queue: VecDeque::new(),
            second_key: String::new(),
            third_key: String::new(),
            fourth_key: String::new(),
            custom_label: String::new(),
            always_visible: false,
            layouts: [Placement::new(
                0.68 + (index % 3) as f32 * 0.1,
                0.35 + (index / 3 % 3) as f32 * 0.12,
                0.105,
            ); 3],
        }
    }

    pub fn valid_token(token: &str) -> bool {
        token == "key.keyboard.unknown"
            || crate::keybinds::key_code(token).is_some()
            || crate::keybinds::mouse_button(token).is_some()
            || matches!(
                token,
                "mouse.scroll.up"
                    | "mouse.scroll.down"
                    | "mouse.virtual.toggle"
                    | "mouse.look"
            )
            || token.strip_prefix("action:").is_some_and(|name| {
                crate::keybinds::MAPPINGS.iter().any(|m| m.0 == name)
            })
    }

    pub fn is_keyboard_binding(token: &str) -> bool {
        !token.is_empty()
            && token != "key.keyboard.unknown"
            && crate::keybinds::key_code(token).is_some()
    }

    pub fn is_mouse_binding(token: &str) -> bool {
        !token.is_empty()
            && token != "key.keyboard.unknown"
            && crate::keybinds::mouse_button(token).is_some()
    }

    /// Sets a single exclusive binding.
    pub fn set_exclusive_binding(&mut self, token: String) {
        self.key = token;
        self.second_key.clear();
        self.third_key.clear();
        self.fourth_key.clear();
    }

    pub fn event_slot(&self, slot: usize) -> &str {
        match slot {
            0 => &self.key,
            1 => &self.second_key,
            2 => &self.third_key,
            _ => &self.fourth_key,
        }
    }

    pub fn set_event_slot(&mut self, slot: usize, token: String) {
        match slot {
            0 => {
                self.key = if token.is_empty() {
                    "key.keyboard.unknown".into()
                } else {
                    token
                };
            }

            1 => {
                if (self.key.is_empty()
                    || self.key == "key.keyboard.unknown")
                    && !token.is_empty()
                    && token != "key.keyboard.unknown"
                {
                    self.key = token;
                } else {
                    self.second_key = token;
                }
            }

            2 => {
                if (self.key.is_empty()
                    || self.key == "key.keyboard.unknown")
                    && !token.is_empty()
                    && token != "key.keyboard.unknown"
                {
                    self.key = token;
                } else {
                    self.third_key = token;
                }
            }

            _ => {
                if (self.key.is_empty()
                    || self.key == "key.keyboard.unknown")
                    && !token.is_empty()
                    && token != "key.keyboard.unknown"
                {
                    self.key = token;
                } else {
                    self.fourth_key = token;
                }
            }
        }
    }

    pub fn token_dispatch_priority(token: &str) -> u8 {
        match token {
            "key.keyboard.1"
            | "key.keyboard.2"
            | "key.keyboard.3"
            | "key.keyboard.4"
            | "key.keyboard.5"
            | "key.keyboard.6"
            | "key.keyboard.7"
            | "key.keyboard.8"
            | "key.keyboard.9"
            | "action:key.hotbar.1"
            | "action:key.hotbar.2"
            | "action:key.hotbar.3"
            | "action:key.hotbar.4"
            | "action:key.hotbar.5"
            | "action:key.hotbar.6"
            | "action:key.hotbar.7"
            | "action:key.hotbar.8"
            | "action:key.hotbar.9"
            | "action:key.swapOffhand"
            | "key.keyboard.f"
            | "mouse.scroll.up"
            | "mouse.scroll.down" => 0,

            "key.keyboard.space"
            | "action:key.jump"
            | "key.keyboard.left.shift"
            | "key.keyboard.right.shift"
            | "action:key.sneak"
            | "key.keyboard.left.control"
            | "key.keyboard.right.control"
            | "action:key.sprint"
            | "key.keyboard.w"
            | "key.keyboard.a"
            | "key.keyboard.s"
            | "key.keyboard.d"
            | "action:key.forward"
            | "action:key.left"
            | "action:key.back"
            | "action:key.right" => 1,

            _ => 2,
        }
    }

    pub fn active_tokens(&self) -> Vec<String> {
        let mut out = Vec::new();

        for token in [
            &self.key,
            &self.second_key,
            &self.third_key,
            &self.fourth_key,
        ] {
            if !token.is_empty()
                && token != "key.keyboard.unknown"
                && Self::valid_token(token)
                && !out.contains(token)
            {
                out.push(token.clone());
            }
        }

        out
    }

    pub fn label(&self) -> String {
        if !self.custom_label.trim().is_empty() {
            return self.custom_label.trim().to_owned();
        }

        let tokens = self.active_tokens();

        if tokens.is_empty() {
            return "Button".into();
        }

        let parts: Vec<String> = tokens
            .iter()
            .map(|token| friendly_token_name(token))
            .collect();

        parts.join("+")
    }
}

pub fn friendly_token_name(token: &str) -> String {
    match token {
        "" | "key.keyboard.unknown" => "Unassigned".into(),

        "key.mouse.left" => "LMB (Attack)".into(),
        "key.mouse.right" => "RMB (Use)".into(),
        "key.mouse.middle" => "MMB (Pick)".into(),
        "key.mouse.4" => "Mouse 4".into(),
        "key.mouse.5" => "Mouse 5".into(),

        "mouse.scroll.up" => "Wheel Up".into(),
        "mouse.scroll.down" => "Wheel Down".into(),
        "mouse.virtual.toggle" => "Virtual Mouse".into(),
        "mouse.look" => "Mouse Look".into(),

        _ => {
            if let Some(name) = token.strip_prefix("action:") {
                let translated =
                    crate::text::Text::translate(name, &[]).plain();

                if translated == name || translated.starts_with("key.") {
                    match name {
                        "key.attack" => "Attack / Mine".into(),
                        "key.use" => "Use / Place".into(),
                        "key.forward" => "Walk Forward".into(),
                        "key.left" => "Strafe Left".into(),
                        "key.back" => "Walk Back".into(),
                        "key.right" => "Strafe Right".into(),
                        "key.jump" => "Jump".into(),
                        "key.sneak" => "Sneak".into(),
                        "key.sprint" => "Sprint".into(),
                        "key.drop" => "Drop Item".into(),
                        "key.inventory" => "Inventory".into(),
                        "key.chat" => "Open Chat".into(),
                        "key.playerlist" => "Player List".into(),
                        "key.pickItem" => "Pick Block".into(),
                        "key.command" => "Command /".into(),
                        "key.toggleGui" => "Hide HUD (F1)".into(),
                        "key.screenshot" => "Screenshot (F2)".into(),
                        "key.togglePerspective" => "Perspective (F5)".into(),
                        "key.swapOffhand" => "Swap Hands (F)".into(),
                        "key.advancements" => "Advancements (L)".into(),
                        "key.debug.overlay" => "Debug Info (F3)".into(),
                        other => other.trim_start_matches("key.").into(),
                    }
                } else {
                    translated
                }
            } else {
                let display =
                    crate::keybinds::display_name(token).plain();

                if display.starts_with("key.keyboard.") {
                    token
                        .trim_start_matches("key.keyboard.")
                        .to_uppercase()
                } else {
                    display
                }
            }
        }
    }
}

pub fn choices(raw_keys: bool) -> Vec<(String, String)> {
    if raw_keys {
        crate::keybinds::available_keys()
            .iter()
            .map(|(name, _)| {
                ((*name).into(), friendly_token_name(name))
            })
            .collect()
    } else {
        crate::keybinds::MAPPINGS
            .iter()
            .filter(|mapping| mapping.1 != "key.keyboard.unknown")
            .map(|(name, _, _)| {
                let token = format!("action:{name}");
                let label = friendly_token_name(&token);
                (token, label)
            })
            .collect()
    }
}

impl TouchControls {
    pub(super) fn inventory_token(
        control: Control,
    ) -> Option<&'static str> {
        match control {
            Control::Sneak => Some("key.keyboard.left.shift"),
            Control::Attack => Some("action:key.attack"),
            Control::Use => Some("action:key.use"),
            _ => None,
        }
    }

    pub fn inventory_update(
        &mut self,
        id: u64,
        point: [f32; 2],
        started: bool,
        ended: bool,
    ) -> bool {
        let has_swipeable = !started
            && !ended
            && self
                .custom_at(point)
                .is_some_and(|index| {
                    self.custom_placement(index).swipeable
                });

        if self.custom_fingers.contains_key(&id)
            || self.custom_swipe_btn.contains_key(&id)
            || (started && self.custom_at(point).is_some())
            || has_swipeable
        {
            return self.custom_update(id, point, started, ended);
        }

        if started
            && self.custom_at(point).is_none()
            && let Some(token) =
                self.control_at(point).and_then(Self::inventory_token)
        {
            let already_held = self
                .custom_fingers
                .values()
                .any(|tokens| tokens.iter().any(|value| value == token));

            self.custom_fingers.insert(id, vec![token.into()]);
            self.fingers.insert(id, (Role::Ignore, point));

            if !already_held {
                self.custom_events.push((token.into(), true));
            }

            return true;
        }

        false
    }

    /// Handles touch input for menu buttons and auxiliary controls.
    pub fn menu_update(
        &mut self,
        id: u64,
        point: [f32; 2],
        started: bool,
        ended: bool,
    ) -> bool {
        let has_swipeable = !started
            && !ended
            && self
                .custom_at(point)
                .is_some_and(|index| {
                    self.custom_placement(index).swipeable
                });

        if self.custom_fingers.contains_key(&id)
            || self.custom_swipe_btn.contains_key(&id)
            || (started && self.custom_at(point).is_some())
            || has_swipeable
        {
            self.custom_update(id, point, started, ended)
        } else {
            self.inventory_update(id, point, started, ended)
        }
    }

    pub fn fit_placement(&self, mut placement: Placement) -> Placement {
        placement.size = (placement.size * self.settings.button_scale)
            .min(self.aspect())
            .min(1.0);

        let half = [
            placement.size / self.aspect() / 2.0,
            placement.size / 2.0,
        ];

        for (index, half_size) in half.into_iter().enumerate() {
            placement.center[index] =
                placement.center[index].clamp(half_size, 1.0 - half_size);
        }

        placement
    }

    pub fn custom_placement(&self, index: usize) -> Placement {
        let Some(button) =
            self.settings.custom_buttons.get(index)
        else {
            return Placement::new(0.5, 0.5, 0.105);
        };

        let mut placement =
            button.layouts[self.settings.mode.index()];

        if self.settings.left_handed {
            placement.center[0] = 1.0 - placement.center[0];
        }

        self.fit_placement(placement)
    }

    pub fn custom_visible_for_state(
        &self,
        index: usize,
        in_menu: bool,
    ) -> bool {
        let Some(button) =
            self.settings.custom_buttons.get(index)
        else {
            return false;
        };

        let placement = self.custom_placement(index);

        let is_goy =
            self.settings.control_style == layout::ControlStyle::GoyControl
                || placement.use_goy_style;

        if placement.opacity <= 0.0 && !is_goy {
            return false;
        }

        if self.editor.is_some() {
            return true;
        }

        if button.always_visible {
            return true;
        }

        if in_menu {
            placement.visible_in_menu
        } else {
            placement.visible_in_game
        }
    }

    pub fn custom_at(&self, point: [f32; 2]) -> Option<usize> {
        (0..self.settings.custom_buttons.len())
            .rev()
            .find(|&index| {
                self.custom_visible_for_state(index, self.in_menu)
                    && layout::contains(
                        self.custom_placement(index)
                            .rect(self.aspect()),
                        point,
                    )
            })
    }

    fn release_custom_finger_key(&mut self, id: u64) {
        self.passthrough_fingers.remove(&id);
        self.custom_swipe_btn.remove(&id);

        let Some(tokens) = self.custom_fingers.remove(&id) else {
            return;
        };

        for token in tokens.into_iter().rev() {
            let still_held = self
                .custom_fingers
                .values()
                .any(|other| other.contains(&token))
                || self.latched_custom.contains(&token);

            if !still_held {
                self.custom_events.push((token, false));
            }
        }
    }

    fn activate_custom_button_at(
        &mut self,
        id: u64,
        index: usize,
        point: [f32; 2],
        is_swipe: bool,
    ) -> bool {
        let Some(button) =
            self.settings.custom_buttons.get(index)
        else {
            return false;
        };

        let active = button.active_tokens();

        if active.is_empty() {
            return false;
        }

        let placement = self.custom_placement(index);

        self.clicked = true;
        self.custom_swipe_btn.insert(id, index);

        if placement.toggleable
            && !active.iter().any(|token| token == "mouse.virtual.toggle")
        {
            let all_latched = active
                .iter()
                .all(|token| self.latched_custom.contains(token));

            let now_on = !all_latched;

            for token in active {
                if now_on {
                    if self.latched_custom.insert(token.clone()) {
                        let already_held = self
                            .custom_fingers
                            .values()
                            .any(|other| other.contains(&token));

                        if !already_held {
                            self.custom_events.push((token, true));
                        }
                    }
                } else if self.latched_custom.remove(&token) {
                    let held_by_finger = self
                        .custom_fingers
                        .values()
                        .any(|other| other.contains(&token));

                    if !held_by_finger {
                        self.custom_events.push((token, false));
                    }
                }
            }

            if !placement.passthrough || self.in_menu {
                if !is_swipe || !self.fingers.contains_key(&id) {
                    self.fingers.insert(
                        id,
                        (Role::Ignore, point),
                    );
                }

                return true;
            }

            self.passthrough_fingers.insert(id);
            return false;
        }

        let mut new_tokens = Vec::with_capacity(active.len());

        for token in &active {
            let already_held = self
                .custom_fingers
                .values()
                .any(|other| other.contains(token))
                || self.latched_custom.contains(token);

            if !already_held {
                self.custom_events.push((token.clone(), true));
            }

            new_tokens.push(token.clone());
        }

        self.custom_fingers.insert(id, new_tokens);

        if placement.passthrough && !self.in_menu {
            self.passthrough_fingers.insert(id);

            if !self
                .fingers
                .get(&id)
                .is_some_and(|(role, _)| {
                    matches!(role, Role::Look { .. })
                })
            {
                self.fingers.insert(
                    id,
                    (
                        Role::Look {
                            start: point,
                            since: Instant::now(),
                            dragged: false,
                            held_use: false,
                            committed: false,
                        },
                        point,
                    ),
                );
            }
        } else if !self
            .fingers
            .get(&id)
            .is_some_and(|(role, _)| {
                matches!(role, Role::Look { .. })
            })
        {
            self.fingers.insert(
                id,
                (Role::Ignore, point),
            );
        }

        true
    }

    pub(super) fn custom_update(
        &mut self,
        id: u64,
        point: [f32; 2],
        started: bool,
        ended: bool,
    ) -> bool {
        if ended {
            self.custom_last_pos.remove(&id);

            let had_swipe =
                self.custom_swipe_btn.remove(&id).is_some();

            if self.custom_fingers.contains_key(&id) {
                self.release_custom_finger_key(id);
                self.fingers.remove(&id);
                return true;
            }

            if had_swipe {
                self.fingers.remove(&id);
                return true;
            }

            return false;
        }

        if started {
            self.custom_last_pos.insert(id, point);

            if let Some(index) = self.custom_at(point) {
                return self.activate_custom_button_at(
                    id,
                    index,
                    point,
                    false,
                );
            }

            return false;
        }

        let previous =
            self.custom_last_pos.insert(id, point).unwrap_or(point);

        let distance = (point[0] - previous[0])
            .hypot(point[1] - previous[1]);

        let steps = ((distance / 0.015).ceil() as usize)
            .clamp(1, 12);

        let mut swiped_any = false;

        for step in 1..=steps {
            let fraction = step as f32 / steps as f32;

            let sample = [
                previous[0]
                    + (point[0] - previous[0]) * fraction,
                previous[1]
                    + (point[1] - previous[1]) * fraction,
            ];

            let hit = self.custom_at(sample);
            let previous_button =
                self.custom_swipe_btn.get(&id).copied();

            if let Some(old_index) = previous_button
                && hit != Some(old_index)
            {
                let old_swipeable =
                    self.custom_placement(old_index).swipeable;

                let new_swipeable = hit.is_some_and(|index| {
                    self.custom_placement(index).swipeable
                });

                if old_swipeable || new_swipeable {
                    self.release_custom_finger_key(id);
                    self.custom_swipe_btn.remove(&id);
                    swiped_any = true;
                }
            }

            if let Some(index) = hit
                && self.custom_placement(index).swipeable
                && self.custom_swipe_btn.get(&id) != Some(&index)
            {
                if self.custom_fingers.contains_key(&id) {
                    self.release_custom_finger_key(id);
                }

                let _ = self.activate_custom_button_at(
                    id,
                    index,
                    sample,
                    true,
                );

                swiped_any = true;
            }
        }

        if self.custom_fingers.contains_key(&id) || swiped_any {
            if !self.in_menu
                && self
                    .fingers
                    .get(&id)
                    .is_some_and(|(role, _)| {
                        matches!(role, Role::Look { .. })
                    })
            {
                return false;
            }

            return true;
        }

        false
    }

    pub fn held_custom_tokens(
        &self,
    ) -> impl Iterator<Item = &str> {
        self.custom_fingers
            .values()
            .flat_map(|tokens| tokens.iter())
            .chain(self.latched_custom.iter())
            .map(String::as_str)
    }

    pub fn custom_button_held(&self, index: usize) -> bool {
        self.custom_swipe_btn
            .values()
            .any(|&button_index| button_index == index)
    }

    pub fn custom_button_toggled(&self, index: usize) -> bool {
        let Some(button) =
            self.settings.custom_buttons.get(index)
        else {
            return false;
        };

        let active = button.active_tokens();

        !active.is_empty()
            && active
                .iter()
                .all(|token| self.latched_custom.contains(token))
    }

    pub fn has_pending_key_events(&self) -> bool {
        !self.custom_events.is_empty()
    }

    pub fn key_events(&mut self) -> Vec<(String, bool)> {
        std::mem::take(&mut self.custom_events)
    }

    pub(super) fn clear_custom(&mut self) {
        self.custom_last_pos.clear();
        self.custom_swipe_btn.clear();
        self.passthrough_fingers.clear();

        let mut all_tokens: std::collections::BTreeSet<String> =
            std::mem::take(&mut self.custom_fingers)
                .into_values()
                .flatten()
                .collect();

        all_tokens.extend(
            std::mem::take(&mut self.latched_custom),
        );

        self.custom_events.extend(
            all_tokens.into_iter().map(|key| (key, false)),
        );
    }

    pub(super) fn draw_custom(
        &self,
        batch: &mut HudBatch,
        atlas: &HudAtlas,
    ) {
        let hud_scale = if self.editor.is_some() {
            1.0
        } else {
            self.hud_opacity_scale.clamp(0.0, 1.0)
        };

        if hud_scale <= 0.001 && self.editor.is_none() {
            return;
        }

        for (index, button) in
            self.settings.custom_buttons.iter().enumerate()
        {
            if self.editor.is_none()
                && !self.custom_visible_for_state(index, self.in_menu)
            {
                continue;
            }

            let placement = self.custom_placement(index);

            let is_goy =
                self.settings.control_style
                    == layout::ControlStyle::GoyControl
                    || placement.use_goy_style;

            if is_goy
                && self.goy_rendered_by_ui
                && self.editor.is_none()
            {
                continue;
            }

            let [nx, ny, nw, nh] =
                placement.rect(self.aspect());

            let rect = [
                nx * batch.width,
                ny * batch.height,
                nw * batch.width,
                nh * batch.height,
            ];

            let held = self.custom_button_held(index);
            let toggled = self.custom_button_toggled(index);
            let pressed = held || toggled;

            let base_opacity = if self.editor.is_some() {
                placement.opacity.max(0.25)
            } else {
                placement.opacity
            };

            let opacity =
                (base_opacity * hud_scale).clamp(0.0, 1.0);

            if opacity <= 0.001 {
                continue;
            }

            if is_goy {
                draw_goy_static_button(
                    batch,
                    atlas,
                    rect,
                    &placement,
                    &button.label(),
                    held,
                    toggled,
                    opacity,
                );
            } else {
                text_button(
                    batch,
                    atlas,
                    rect,
                    &button.label(),
                    pressed,
                    rgba(
                        255,
                        255,
                        255,
                        (opacity * 255.0).round() as u8,
                    ),
                    true,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unassigned_button_emits_no_input() {
        let mut touch = TouchControls::default();
        touch.viewport(1920, 1080);

        touch.settings.custom_buttons.push(
            CustomButton::new(
                "key.keyboard.unknown".into(),
                0,
            ),
        );

        let point = touch.custom_placement(0).center;

        touch.update(1, point, true, false);

        assert!(touch.key_events().is_empty());
    }

    #[test]
    fn custom_button_supports_multiple_keys() {
        let mut touch = TouchControls::default();
        touch.viewport(1920, 1080);

        let mut button =
            CustomButton::new("key.keyboard.z".into(), 0);

        button.set_event_slot(
            1,
            "key.keyboard.x".into(),
        );

        button.set_event_slot(
            2,
            "key.mouse.left".into(),
        );

        touch.settings.custom_buttons.push(button);

        let point = touch.custom_placement(0).center;

        touch.update(1, point, true, false);

        assert_eq!(touch.key_events().len(), 3);

        touch.clear();

        let released = touch.key_events();

        assert_eq!(released.len(), 3);
    }

    #[test]
    fn custom_button_label_works() {
        let mut button =
            CustomButton::new("key.keyboard.z".into(), 0);

        assert_eq!(button.label(), "Z");

        button.custom_label = "My Button".into();

        assert_eq!(button.label(), "My Button");
    }

    #[test]
    fn shared_keys_release_after_last_finger() {
        let mut touch = TouchControls::default();
        touch.viewport(1920, 1080);

        touch.settings.custom_buttons.push(
            CustomButton::new(
                "key.keyboard.z".into(),
                0,
            ),
        );

        let point = touch.custom_placement(0).center;

        touch.update(1, point, true, false);
        touch.update(2, point, true, false);

        assert_eq!(
            touch.key_events(),
            [("key.keyboard.z".into(), true)]
        );

        touch.update(1, [0.99, 0.1], false, false);
        touch.update(1, point, false, true);

        assert!(touch.key_events().is_empty());

        touch.update(2, point, false, true);

        assert_eq!(
            touch.key_events(),
            [("key.keyboard.z".into(), false)]
        );
    }
                        }
