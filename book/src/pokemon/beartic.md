<details class="pokemon-card-container">
<summary>Beartic (#375)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-beartic">
<input type="radio" name="pokemon-tabs-beartic-group" id="pokemon-tabs-beartic-tab-0">
<label for="pokemon-tabs-beartic-tab-0">Cubchoo</label>
<input type="radio" name="pokemon-tabs-beartic-group" id="pokemon-tabs-beartic-tab-1" checked>
<label for="pokemon-tabs-beartic-tab-1">Beartic</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-beartic-panel-0">
Types: Ice • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Snow Cloak
- Slush Rush
- Fur Coat *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Ice (0.5×)

*Weak to*
- Fire (2×)
- Fighting (2×)
- Rock (2×)
- Steel (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM12 - Taunt
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM28 - Dig
- TM32 - Double Team
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- TM52 - Frost Breath
- TM53 - Power-Up Punch
- TM55 - Snarl
- HM01 - Cut
- HM03 - Surf
- HM04 - Strength
- HM06 - Rock Smash

**Encounter Locations**
- Froslass Cavern BF2 — Grass (Day) (10%)
- Riverwalk Trail (West) — Grass (Night) (8%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="cubchoo" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-mid">70</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-low">35</span> |
| Total | <span class="stat-value stat-low">305</span> |

**Level-Up Moves**
- Growl (Lv 1)
- Powder Snow (Lv 5)
- Bide (Lv 9)
- Icy Wind (Lv 13)
- Play Nice (Lv 15)
- Fury Swipes (Lv 17)
- Brine (Lv 21)
- Charm (Lv 25)
- Avalanche (Lv 29)
- Slash (Lv 33)
- Flail (Lv 36)
- Liquidation (Lv 40)
- Rest (Lv 42)
- Crush Claw (Lv 45)
- Blizzard (Lv 47)
- Hail (Lv 49)
- Thrash (Lv 53)
- Sheer Cold (Lv 57)

**Egg Moves**
- Yawn
- Avalanche
- Encore
- Ice Punch
- Night Slash
- Assurance
- Sleep Talk
- Focus Punch
- Play Rough

**Tutor Moves**
- Body Slam
- Endure
- Ice Punch
- Icy Wind
- Mega Kick
- Mega Punch
- Mud-Slap
- Rock Slide
- Sleep Talk
- Snore
- Swagger
</div>
</div>
<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>
</div>
<div class="pokemon-tab-panel" id="pokemon-tabs-beartic-panel-1">
Types: Ice • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Snow Cloak
- Slush Rush
- Fur Coat *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Ice (0.5×)

*Weak to*
- Fire (2×)
- Fighting (2×)
- Rock (2×)
- Steel (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM08 - Bulk Up
- TM12 - Taunt
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM26 - Earthquake
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- TM52 - Frost Breath
- TM53 - Power-Up Punch
- TM55 - Snarl
- HM01 - Cut
- HM03 - Surf
- HM04 - Strength
- HM06 - Rock Smash
- HM08 - Dive

**Evolution Info**
Lv. 37
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="beartic" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">100</span> |
| Attack | <span class="stat-value stat-high">130</span> |
| Defense | <span class="stat-value stat-mid">80</span> |
| Sp. Atk | <span class="stat-value stat-mid">70</span> |
| Sp. Def | <span class="stat-value stat-mid">85</span> |
| Speed | <span class="stat-value stat-low">50</span> |
| Total | <span class="stat-value stat-mid">515</span> |

**Level-Up Moves**
- Icicle Crash (Lv Evolve)
- Sheer Cold (Lv 1)
- Thrash (Lv 1)
- Superpower (Lv 1)
- Aqua Jet (Lv 1)
- Growl (Lv 1)
- Powder Snow (Lv 5)
- Bide (Lv 9)
- Icy Wind (Lv 13)
- Play Nice (Lv 15)
- Fury Swipes (Lv 17)
- Brine (Lv 21)
- Charm (Lv 25)
- Avalanche (Lv 29)
- Slash (Lv 33)
- Flail (Lv 36)
- Liquidation (Lv 40)
- Rest (Lv 42)
- Crush Claw (Lv 45)
- Blizzard (Lv 47)
- Hail (Lv 49)
- Thrash (Lv 53)
- Sheer Cold (Lv 57)

**Egg Moves**
- Yawn
- Avalanche
- Encore
- Ice Punch
- Night Slash
- Assurance
- Sleep Talk
- Focus Punch
- Play Rough

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Ice Punch
- Icy Wind
- Mega Kick
- Mega Punch
- Mud-Slap
- Rock Slide
- Sleep Talk
- Snore
- Swagger
- Swords Dance
</div>
</div>
<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>
</div>
</div>
</div>
<style>
#pokemon-tabs-beartic-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-beartic-panel-0 { display: block; }
#pokemon-tabs-beartic-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-beartic-panel-1 { display: block; }
</style>
</details>
