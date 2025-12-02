<details class="pokemon-card-container">
<summary>Arbok (#286)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-arbok">
<input type="radio" name="pokemon-tabs-arbok-group" id="pokemon-tabs-arbok-tab-0">
<label for="pokemon-tabs-arbok-tab-0">Ekans</label>
<input type="radio" name="pokemon-tabs-arbok-group" id="pokemon-tabs-arbok-tab-1" checked>
<label for="pokemon-tabs-arbok-tab-1">Arbok</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-arbok-panel-0">
Types: Poison / Dark • Egg Groups: Field / Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Intimidate
- Shed Skin
- Strong Jaw *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Poison (0.5×)
- Psychic (0×)
- Ghost (0.5×)
- Dark (0.5×)

*Weak to*
- Ground (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM20 - Poison Jab
- TM26 - Earthquake
- TM28 - Dig
- TM32 - Double Team
- TM36 - Sludge Bomb
- TM39 - Rock Tomb
- TM41 - Torment
- TM42 - Facade
- TM43 - Poison Fang
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- TM55 - Snarl
- TM59 - Dark Pulse
- HM04 - Strength

**Encounter Locations**
- Pythios Cemetery — Grass (Night) (20%)
- Riverwalk Trail (South) — Grass (Day) (10%)
- Riverwalk Trail (South) — Grass (Night) (10%)
- Sea of Asteri (West) — Grass (Night) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="ekans" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">35</span> |
| Attack | <span class="stat-value stat-mid">60</span> |
| Defense | <span class="stat-value stat-low">44</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-mid">54</span> |
| Speed | <span class="stat-value stat-mid">55</span> |
| Total | <span class="stat-value stat-low">288</span> |

**Level-Up Moves**
- Wrap (Lv 1)
- Leer (Lv 1)
- Poison Sting (Lv 4)
- Bite (Lv 9)
- Glare (Lv 12)
- Screech (Lv 17)
- Acid (Lv 20)
- Stockpile (Lv 23)
- Swallow (Lv 23)
- Spit Up (Lv 23)
- Poison Fang (Lv 26)
- Acid Spray (Lv 28)
- Mud Bomb (Lv 33)
- Gastro Acid (Lv 36)
- Belch (Lv 38)
- Haze (Lv 41)
- Coil (Lv 44)
- Gunk Shot (Lv 49)

**Egg Moves**
- Pursuit
- Slam
- Spite
- Beat Up
- Poison Fang
- Scary Face
- Poison Tail
- Disable
- Switcheroo
- Iron Tail
- Sucker Punch
- Snatch

**Tutor Moves**
- Acid Spray
- Body Slam
- Double-Edge
- Endure
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
<div class="pokemon-tab-panel" id="pokemon-tabs-arbok-panel-1">
Types: Poison / Dark • Egg Groups: Field / Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Intimidate
- Shed Skin
- Strong Jaw *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Poison (0.5×)
- Psychic (0×)
- Ghost (0.5×)
- Dark (0.5×)

*Weak to*
- Ground (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM20 - Poison Jab
- TM26 - Earthquake
- TM28 - Dig
- TM32 - Double Team
- TM36 - Sludge Bomb
- TM39 - Rock Tomb
- TM41 - Torment
- TM42 - Facade
- TM43 - Poison Fang
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- TM55 - Snarl
- TM59 - Dark Pulse
- HM04 - Strength

**Evolution Info**
Lv. 22

**Encounter Locations**
- Kaptara Island (East) — Grass (Night) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="arbok" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-high">95</span> |
| Defense | <span class="stat-value stat-mid">74</span> |
| Sp. Atk | <span class="stat-value stat-mid">65</span> |
| Sp. Def | <span class="stat-value stat-mid">79</span> |
| Speed | <span class="stat-value stat-mid">80</span> |
| Total | <span class="stat-value stat-mid">463</span> |

**Level-Up Moves**
- Crunch (Lv Evo)
- Ice Fang (Lv 1)
- Thunder Fang (Lv 1)
- Fire Fang (Lv 1)
- Wrap (Lv 1)
- Leer (Lv 1)
- Poison Sting (Lv 4)
- Bite (Lv 9)
- Glare (Lv 12)
- Screech (Lv 17)
- Acid (Lv 20)
- Stockpile (Lv 23)
- Swallow (Lv 23)
- Spit Up (Lv 23)
- Poison Fang (Lv 26)
- Acid Spray (Lv 28)
- Mud Bomb (Lv 33)
- Gastro Acid (Lv 36)
- Belch (Lv 38)
- Haze (Lv 41)
- Coil (Lv 44)
- Gunk Shot (Lv 49)

**Egg Moves**
- Pursuit
- Slam
- Spite
- Beat Up
- Poison Fang
- Scary Face
- Poison Tail
- Disable
- Switcheroo
- Iron Tail
- Sucker Punch
- Snatch

**Tutor Moves**
- Acid Spray
- Body Slam
- Double-Edge
- Endure
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
</div>
</div>
<style>
#pokemon-tabs-arbok-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-arbok-panel-0 { display: block; }
#pokemon-tabs-arbok-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-arbok-panel-1 { display: block; }
</style>
</details>
