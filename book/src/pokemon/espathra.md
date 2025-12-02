<details class="pokemon-card-container">
<summary>Espathra (#146)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-espathra">
<input type="radio" name="pokemon-tabs-espathra-group" id="pokemon-tabs-espathra-tab-0">
<label for="pokemon-tabs-espathra-tab-0">Flittle</label>
<input type="radio" name="pokemon-tabs-espathra-group" id="pokemon-tabs-espathra-tab-1" checked>
<label for="pokemon-tabs-espathra-tab-1">Espathra</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-espathra-panel-0">
Types: Psychic • Egg Groups: Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Anticipation
- Frisk
- Speed Boost *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fighting (0.5×)
- Psychic (0.5×)

*Weak to*
- Bug (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM05 - Psyshock
- TM11 - Sunny Day
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM29 - Psychic
- TM33 - Reflect
- TM37 - Sandstorm
- TM42 - Facade
- TM44 - Rest
- TM46 - Thief
- TM48 - Skill Swap
- TM54 - Dazzling Gleam

**Encounter Locations**
- Acrisia Mountains (Peak) — Grass (Day) (20%)
- Acrisia Mountains (Peak) — Grass (Night) (20%)
- Pythios Town — Grass (Day) (10%)
- Pythios Town — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="flittle" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">40</span> |
| Attack | <span class="stat-value stat-low">35</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-mid">55</span> |
| Sp. Def | <span class="stat-value stat-low">44</span> |
| Speed | <span class="stat-value stat-mid">71</span> |
| Total | <span class="stat-value stat-low">285</span> |

**Level-Up Moves**
- Growl (Lv 1)
- Peck (Lv 1)
- Confusion (Lv 5)
- Baby-Doll Eyes (Lv 8)
- Disarming Voice (Lv 11)
- Quick Attack (Lv 14)
- Teeter Dance (Lv 17)
- Psybeam (Lv 19)
- Pluck (Lv 24)
- Agility (Lv 29)
- Uproar (Lv 34)

**Egg Moves**
- Ally Switch
- Hypnosis
- Roost

**Tutor Moves**
- Endure
- Mud-Slap
- Sleep Talk
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
<div class="pokemon-tab-panel" id="pokemon-tabs-espathra-panel-1">
Types: Psychic • Egg Groups: Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Opportunist
- Frisk
- Speed Boost *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fighting (0.5×)
- Psychic (0.5×)

*Weak to*
- Bug (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM05 - Psyshock
- TM11 - Sunny Day
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM23 - Hex
- TM29 - Psychic
- TM30 - Shadow Ball
- TM33 - Reflect
- TM37 - Sandstorm
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM46 - Thief
- TM48 - Skill Swap
- TM54 - Dazzling Gleam
- TM57 - Roost

**Evolution Info**
Lv. 30

**Encounter Locations**
- Kaptara Island (West) — Grass (Night) (10%)
- Pollen Road — Grass (Night) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="espathra" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">95</span> |
| Attack | <span class="stat-value stat-mid">60</span> |
| Defense | <span class="stat-value stat-mid">70</span> |
| Sp. Atk | <span class="stat-value stat-high">101</span> |
| Sp. Def | <span class="stat-value stat-mid">74</span> |
| Speed | <span class="stat-value stat-high">101</span> |
| Total | <span class="stat-value stat-mid">501</span> |

**Level-Up Moves**
- Lumina Crash (Lv Evo)
- Growl (Lv 1)
- Peck (Lv 1)
- Drill Peck (Lv 1)
- Feather Dance (Lv 1)
- Confusion (Lv 5)
- Baby-Doll Eyes (Lv 8)
- Disarming Voice (Lv 11)
- Quick Attack (Lv 14)
- Teeter Dance (Lv 17)
- Psybeam (Lv 19)
- Pluck (Lv 24)
- Agility (Lv 29)
- Uproar (Lv 34)
- Dazzling Gleam (Lv 37)
- Psychic (Lv 43)
- No Retreat (Lv 45)
- Last Resort (Lv 50)

**Egg Moves**
- Ally Switch
- Hypnosis
- Roost

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Mud-Slap
- Psych Up
- Sleep Talk
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
#pokemon-tabs-espathra-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-espathra-panel-0 { display: block; }
#pokemon-tabs-espathra-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-espathra-panel-1 { display: block; }
</style>
</details>
