<details class="pokemon-card-container">
<summary>Pansage (#028)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-pansage">
<input type="radio" name="pokemon-tabs-pansage-group" id="pokemon-tabs-pansage-tab-0" checked>
<label for="pokemon-tabs-pansage-tab-0">Pansage</label>
<input type="radio" name="pokemon-tabs-pansage-group" id="pokemon-tabs-pansage-tab-1">
<label for="pokemon-tabs-pansage-tab-1">Simisage</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-pansage-panel-0">
Types: Grass / Normal • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Gluttony
- Cud Chew
- Overgrow *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Ground (0.5×)
- Ghost (0×)

*Weak to*
- Fire (2×)
- Ice (2×)
- Fighting (2×)
- Poison (2×)
- Flying (2×)
- Bug (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM09 - Bullet Seed
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM28 - Dig
- TM32 - Double Team
- TM39 - Rock Tomb
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- HM01 - Cut
- HM05 - Flash
- HM06 - Rock Smash

**Encounter Locations**
- Acrisia City — Grass (Day) (5%)
- Acrisia City — Grass (Night) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="pansage" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">50</span> |
| Attack | <span class="stat-value stat-mid">58</span> |
| Defense | <span class="stat-value stat-mid">53</span> |
| Sp. Atk | <span class="stat-value stat-mid">58</span> |
| Sp. Def | <span class="stat-value stat-mid">53</span> |
| Speed | <span class="stat-value stat-mid">64</span> |
| Total | <span class="stat-value stat-mid">336</span> |

**Level-Up Moves**
- Scratch (Lv 1)
- Play Nice (Lv 1)
- Leer (Lv 4)
- Lick (Lv 7)
- Vine Whip (Lv 10)
- Fury Swipes (Lv 13)
- Leech Seed (Lv 16)
- Bite (Lv 19)
- Seed Bomb (Lv 22)
- Torment (Lv 25)
- Fling (Lv 28)
- Slam (Lv 30)
- Acrobatics (Lv 32)
- Grass Knot (Lv 34)
- Recycle (Lv 37)
- Natural Gift (Lv 40)
- Crunch (Lv 43)
- Spiky Shield (Lv 46)

**Egg Moves**
- Covet
- Low Kick
- Tickle
- Nasty Plot
- Role Play
- Astonish
- Grass Whistle
- Magical Leaf
- Bullet Seed
- Leaf Storm
- Disarming Voice
- Spiky Shield

**Tutor Moves**
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
<div class="pokemon-tab-panel" id="pokemon-tabs-pansage-panel-1">
Types: Grass / Normal • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Gluttony
- Cud Chew
- Overgrow *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Ground (0.5×)
- Ghost (0×)

*Weak to*
- Fire (2×)
- Ice (2×)
- Fighting (2×)
- Poison (2×)
- Flying (2×)
- Bug (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM09 - Bullet Seed
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM39 - Rock Tomb
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM53 - Power-Up Punch
- HM01 - Cut
- HM05 - Flash
- HM06 - Rock Smash

**Evolution Info**
Leaf Stone
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="simisage" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">75</span> |
| Attack | <span class="stat-value stat-high">103</span> |
| Defense | <span class="stat-value stat-mid">63</span> |
| Sp. Atk | <span class="stat-value stat-high">98</span> |
| Sp. Def | <span class="stat-value stat-mid">63</span> |
| Speed | <span class="stat-value stat-high">103</span> |
| Total | <span class="stat-value stat-mid">505</span> |

**Level-Up Moves**
- Scratch (Lv 1)
- Play Nice (Lv 1)
- Leer (Lv 4)
- Lick (Lv 7)
- Vine Whip (Lv 10)
- Fury Swipes (Lv 13)
- Leech Seed (Lv 16)
- Bite (Lv 19)
- Seed Bomb (Lv 22)
- Torment (Lv 25)
- Fling (Lv 28)
- Slam (Lv 30)
- Acrobatics (Lv 32)
- Grass Knot (Lv 34)
- Recycle (Lv 37)
- Natural Gift (Lv 40)
- Crunch (Lv 43)
- Spiky Shield (Lv 46)

**Egg Moves**
- Covet
- Low Kick
- Tickle
- Nasty Plot
- Role Play
- Astonish
- Grass Whistle
- Magical Leaf
- Bullet Seed
- Leaf Storm
- Disarming Voice
- Spiky Shield

**Tutor Moves**
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
#pokemon-tabs-pansage-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-pansage-panel-0 { display: block; }
#pokemon-tabs-pansage-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-pansage-panel-1 { display: block; }
</style>
</details>
