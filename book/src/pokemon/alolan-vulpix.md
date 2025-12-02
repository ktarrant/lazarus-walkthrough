<details class="pokemon-card-container">
<summary>Alolan Vulpix (#147)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-alolan-vulpix">
<input type="radio" name="pokemon-tabs-alolan-vulpix-group" id="pokemon-tabs-alolan-vulpix-tab-0" checked>
<label for="pokemon-tabs-alolan-vulpix-tab-0">Alolan Vulpix</label>
<input type="radio" name="pokemon-tabs-alolan-vulpix-group" id="pokemon-tabs-alolan-vulpix-tab-1">
<label for="pokemon-tabs-alolan-vulpix-tab-1">Alolan Ninetales</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-alolan-vulpix-panel-0">
Types: Ice / Fairy • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Snow Cloak
- Snow Warning *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Ice (0.5×)
- Bug (0.5×)
- Dragon (0×)
- Dark (0.5×)

*Weak to*
- Fire (2×)
- Poison (2×)
- Rock (2×)
- Steel (4×)

**TM/HM Moves**
- TM06 - Toxic
- TM13 - Ice Beam
- TM14 - Blizzard
- TM15 - Draining Kiss
- TM17 - Protect
- TM18 - Rain Dance
- TM23 - Hex
- TM28 - Dig
- TM32 - Double Team
- TM33 - Reflect
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM50 - Deepwater Curse
- TM52 - Frost Breath
- TM54 - Dazzling Gleam
- TM59 - Dark Pulse

**Held Item**
Snowball

**Encounter Locations**
- Froslass Cavern BF1 — Grass (Day) (20%)
- Jusmail Town — Grass (Night) (4%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="alolan-vulpix" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">38</span> |
| Attack | <span class="stat-value stat-low">41</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-mid">65</span> |
| Speed | <span class="stat-value stat-mid">65</span> |
| Total | <span class="stat-value stat-mid">309</span> |

**Level-Up Moves**
- Powder Snow (Lv 1)
- Tail Whip (Lv 4)
- Roar (Lv 7)
- Baby-Doll Eyes (Lv 9)
- Ice Shard (Lv 10)
- Confuse Ray (Lv 12)
- Icy Wind (Lv 15)
- Payback (Lv 18)
- Deepwater Curse (Lv 20)
- Feint Attack (Lv 23)
- Hex (Lv 26)
- Aurora Beam (Lv 28)
- Extrasensory (Lv 31)
- Safeguard (Lv 34)
- Ice Beam (Lv 36)
- Imprison (Lv 39)
- Blizzard (Lv 42)
- Aqua Step (Lv 44)
- Captivate (Lv 47)
- Sheer Cold (Lv 50)
- Moonblast (Lv 52)

**Egg Moves**
- Agility
- Charm
- Disable
- Encore
- Extrasensory
- Flail
- Freeze-Dry
- Howl
- Hypnosis
- Moonblast
- Power Swap
- Spite
- Secret Power
- Tail Slap

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Icy Wind
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
<div class="pokemon-tab-panel" id="pokemon-tabs-alolan-vulpix-panel-1">
Types: Ice / Fairy • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Snow Cloak
- Snow Warning *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Ice (0.5×)
- Bug (0.5×)
- Dragon (0×)
- Dark (0.5×)

*Weak to*
- Fire (2×)
- Poison (2×)
- Rock (2×)
- Steel (4×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM05 - Psyshock
- TM06 - Toxic
- TM13 - Ice Beam
- TM14 - Blizzard
- TM15 - Draining Kiss
- TM17 - Protect
- TM18 - Rain Dance
- TM22 - Solar Beam
- TM23 - Hex
- TM28 - Dig
- TM32 - Double Team
- TM33 - Reflect
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM50 - Deepwater Curse
- TM52 - Frost Breath
- TM54 - Dazzling Gleam
- TM59 - Dark Pulse

**Held Item**
Snowball

**Evolution Info**
Ice Stone
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="alolan-ninetales" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">83</span> |
| Attack | <span class="stat-value stat-mid">67</span> |
| Defense | <span class="stat-value stat-mid">75</span> |
| Sp. Atk | <span class="stat-value stat-high">96</span> |
| Sp. Def | <span class="stat-value stat-high">100</span> |
| Speed | <span class="stat-value stat-high">114</span> |
| Total | <span class="stat-value stat-mid">535</span> |

**Level-Up Moves**
- Dazzling Gleam (Lv Evo)
- Imprison (Lv 1)
- Nasty Plot (Lv 1)
- Ice Beam (Lv 1)
- Ice Shard (Lv 1)
- Powder Snow (Lv 1)
- Tail Whip (Lv 4)
- Roar (Lv 7)
- Baby-Doll Eyes (Lv 9)
- Ice Shard (Lv 10)
- Confuse Ray (Lv 12)
- Icy Wind (Lv 15)
- Payback (Lv 18)
- Deepwater Curse (Lv 20)
- Feint Attack (Lv 23)
- Hex (Lv 26)
- Aurora Beam (Lv 28)
- Extrasensory (Lv 31)
- Safeguard (Lv 34)
- Ice Beam (Lv 36)
- Imprison (Lv 39)
- Blizzard (Lv 42)
- Aqua Step (Lv 44)
- Captivate (Lv 47)
- Sheer Cold (Lv 50)
- Moonblast (Lv 52)

**Egg Moves**
- Agility
- Charm
- Disable
- Encore
- Extrasensory
- Flail
- Freeze-Dry
- Howl
- Hypnosis
- Moonblast
- Power Swap
- Spite
- Secret Power
- Tail Slap

**Tutor Moves**
- Body Slam
- Double-Edge
- Dream Eater
- Endure
- Icy Wind
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
#pokemon-tabs-alolan-vulpix-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-alolan-vulpix-panel-0 { display: block; }
#pokemon-tabs-alolan-vulpix-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-alolan-vulpix-panel-1 { display: block; }
</style>
</details>
