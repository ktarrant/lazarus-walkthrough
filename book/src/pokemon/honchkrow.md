<details class="pokemon-card-container">
<summary>Honchkrow (#095)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-honchkrow">
<input type="radio" name="pokemon-tabs-honchkrow-group" id="pokemon-tabs-honchkrow-tab-0">
<label for="pokemon-tabs-honchkrow-tab-0">Murkrow</label>
<input type="radio" name="pokemon-tabs-honchkrow-group" id="pokemon-tabs-honchkrow-tab-1" checked>
<label for="pokemon-tabs-honchkrow-tab-1">Honchkrow</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-honchkrow-panel-0">
Types: Dark / Flying • Egg Groups: Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Insomnia
- Super Luck
- Prankster *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Ground (0×)
- Psychic (0×)
- Ghost (0.5×)
- Dark (0.5×)

*Weak to*
- Electric (2×)
- Ice (2×)
- Rock (2×)
- Fairy (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM06 - Toxic
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM23 - Hex
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM40 - Aerial Ace
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM47 - Steel Wing
- TM55 - Snarl
- TM57 - Roost
- TM58 - Thunder Wave
- TM59 - Dark Pulse
- HM02 - Fly

**Encounter Locations**
- Bronze Fields (North) — Grass (Night) (5%)
- Bronze Fields (South) — Grass (Night) (20%)
- Kalami City — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="murkrow" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">65</span> |
| Attack | <span class="stat-value stat-mid">85</span> |
| Defense | <span class="stat-value stat-low">42</span> |
| Sp. Atk | <span class="stat-value stat-mid">85</span> |
| Sp. Def | <span class="stat-value stat-low">42</span> |
| Speed | <span class="stat-value stat-high">91</span> |
| Total | <span class="stat-value stat-mid">410</span> |

**Level-Up Moves**
- Peck (Lv 1)
- Astonish (Lv 1)
- Pursuit (Lv 5)
- Haze (Lv 11)
- Wing Attack (Lv 15)
- Thief (Lv 18)
- Night Shade (Lv 21)
- Assurance (Lv 25)
- Dual Wingbeat (Lv 29)
- Taunt (Lv 31)
- Feint Attack (Lv 35)
- Snarl (Lv 38)
- Mean Look (Lv 41)
- Foul Play (Lv 44)
- Night Slash (Lv 45)
- Tailwind (Lv 50)
- Sucker Punch (Lv 55)
- Torment (Lv 61)
- Quash (Lv 65)

**Egg Moves**
- Whirlwind
- Drill Peck
- Mirror Move
- Wing Attack
- Sky Attack
- Confuse Ray
- Feather Dance
- Perish Song
- Psycho Shift
- Screech
- Feint Attack
- Brave Bird
- Roost
- Assurance
- Flatter
- Punishment

**Tutor Moves**
- Double-Edge
- Dream Eater
- Endure
- Icy Wind
- Mud-Slap
- Psych Up
- Sleep Talk
- Snore
- Swagger
- Swift
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
<div class="pokemon-tab-panel" id="pokemon-tabs-honchkrow-panel-1">
Types: Dark / Flying • Egg Groups: Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Moxie
- Super Luck
- Prankster *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Ground (0×)
- Psychic (0×)
- Ghost (0.5×)
- Dark (0.5×)

*Weak to*
- Electric (2×)
- Ice (2×)
- Rock (2×)
- Fairy (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM06 - Toxic
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM23 - Hex
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM40 - Aerial Ace
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM47 - Steel Wing
- TM55 - Snarl
- TM57 - Roost
- TM58 - Thunder Wave
- TM59 - Dark Pulse
- HM02 - Fly

**Evolution Info**
Dusk Stone
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="honchkrow" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">100</span> |
| Attack | <span class="stat-value stat-high">125</span> |
| Defense | <span class="stat-value stat-mid">52</span> |
| Sp. Atk | <span class="stat-value stat-high">105</span> |
| Sp. Def | <span class="stat-value stat-mid">52</span> |
| Speed | <span class="stat-value stat-mid">71</span> |
| Total | <span class="stat-value stat-mid">505</span> |

**Level-Up Moves**
- Night Slash (Lv Evo)
- Peck (Lv 1)
- Astonish (Lv 1)
- Pursuit (Lv 5)
- Haze (Lv 11)
- Wing Attack (Lv 15)
- Thief (Lv 18)
- Night Shade (Lv 21)
- Assurance (Lv 25)
- Dual Wingbeat (Lv 29)
- Taunt (Lv 31)
- Feint Attack (Lv 35)
- Snarl (Lv 38)
- Mean Look (Lv 41)
- Foul Play (Lv 44)
- Night Slash (Lv 45)
- Tailwind (Lv 50)
- Sucker Punch (Lv 55)
- Torment (Lv 61)
- Quash (Lv 65)

**Egg Moves**
- Whirlwind
- Drill Peck
- Mirror Move
- Wing Attack
- Sky Attack
- Confuse Ray
- Feather Dance
- Perish Song
- Psycho Shift
- Screech
- Feint Attack
- Brave Bird
- Roost
- Assurance
- Flatter
- Punishment

**Tutor Moves**
- Double-Edge
- Dream Eater
- Endure
- Icy Wind
- Mud-Slap
- Psych Up
- Sleep Talk
- Snore
- Swagger
- Swift
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
#pokemon-tabs-honchkrow-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-honchkrow-panel-0 { display: block; }
#pokemon-tabs-honchkrow-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-honchkrow-panel-1 { display: block; }
</style>
</details>
