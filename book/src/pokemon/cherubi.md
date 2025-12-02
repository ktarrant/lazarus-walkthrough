<details class="pokemon-card-container">
<summary>Cherubi (#167)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-cherubi">
<input type="radio" name="pokemon-tabs-cherubi-group" id="pokemon-tabs-cherubi-tab-0" checked>
<label for="pokemon-tabs-cherubi-tab-0">Cherubi</label>
<input type="radio" name="pokemon-tabs-cherubi-group" id="pokemon-tabs-cherubi-tab-1">
<label for="pokemon-tabs-cherubi-tab-1">Cherrim</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-cherubi-panel-0">
Types: Grass • Egg Groups: Grass / Fairy

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Chlorophyll

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (2×)
- Ice (2×)
- Poison (2×)
- Flying (2×)
- Bug (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM09 - Bullet Seed
- TM11 - Sunny Day
- TM15 - Draining Kiss
- TM17 - Protect
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM32 - Double Team
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM54 - Dazzling Gleam
- HM05 - Flash

**Held Item**
Miracle Seed

**Encounter Locations**
- Erinys Path (East) — Grass (Day) (20%)
- Myrrini Island — Grass (Day) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="cherubi" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">45</span> |
| Attack | <span class="stat-value stat-low">35</span> |
| Defense | <span class="stat-value stat-low">45</span> |
| Sp. Atk | <span class="stat-value stat-mid">62</span> |
| Sp. Def | <span class="stat-value stat-mid">53</span> |
| Speed | <span class="stat-value stat-low">35</span> |
| Total | <span class="stat-value stat-low">275</span> |

**Level-Up Moves**
- Morning Sun (Lv 1)
- Tackle (Lv 1)
- Growth (Lv 7)
- Leech Seed (Lv 10)
- Helping Hand (Lv 13)
- Magical Leaf (Lv 19)
- Sunny Day (Lv 22)
- Worry Seed (Lv 28)
- Take Down (Lv 31)
- Solar Beam (Lv 37)
- Lucky Chant (Lv 40)
- Petal Blizzard (Lv 47)

**Egg Moves**
- Razor Leaf
- Sweet Scent
- Tickle
- Nature Power
- Grass Whistle
- Aromatherapy
- Weather Ball
- Heal Pulse
- Healing Wish
- Seed Bomb
- Natural Gift
- Defense Curl
- Rollout
- Flower Shield
- Grassy Terrain

**Tutor Moves**
- Defense Curl
- Double-Edge
- Endure
- Rollout
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
<div class="pokemon-tab-panel" id="pokemon-tabs-cherubi-panel-1">
Types: Grass / Fire • Egg Groups: Grass / Fairy

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Flower Gift
- Drought *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Grass (0.25×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Poison (2×)
- Flying (2×)
- Rock (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM09 - Bullet Seed
- TM11 - Sunny Day
- TM15 - Draining Kiss
- TM17 - Protect
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM32 - Double Team
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM54 - Dazzling Gleam
- HM05 - Flash

**Held Item**
Miracle Seed

**Evolution Info**
Lv. 25

**Encounter Locations**
- Kaptara Island (East) — Grass (Day) (4%)
- Lastlight Road — Grass (Day) (10%)
- Port Pello — Grass (Day) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="cherrim" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-mid">60</span> |
| Defense | <span class="stat-value stat-mid">80</span> |
| Sp. Atk | <span class="stat-value stat-high">97</span> |
| Sp. Def | <span class="stat-value stat-mid">88</span> |
| Speed | <span class="stat-value stat-mid">85</span> |
| Total | <span class="stat-value stat-mid">480</span> |

**Level-Up Moves**
- Petal Dance (Lv Evo)
- Sunny Day (Lv Evo)
- Morning Sun (Lv 1)
- Tackle (Lv 1)
- Growth (Lv 7)
- Leech Seed (Lv 10)
- Helping Hand (Lv 13)
- Magical Leaf (Lv 19)
- Sunny Day (Lv 22)
- Flame Burst (Lv 27)
- Worry Seed (Lv 30)
- Flame Burst (Lv 35)
- Solar Beam (Lv 38)
- Fiery Dance (Lv 43)
- Lucky Chant (Lv 48)
- Petal Blizzard (Lv 50)

**Egg Moves**
- Razor Leaf
- Sweet Scent
- Tickle
- Nature Power
- Grass Whistle
- Aromatherapy
- Weather Ball
- Heal Pulse
- Healing Wish
- Seed Bomb
- Natural Gift
- Defense Curl
- Rollout
- Flower Shield
- Grassy Terrain

**Tutor Moves**
- Defense Curl
- Double-Edge
- Endure
- Rollout
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
#pokemon-tabs-cherubi-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-cherubi-panel-0 { display: block; }
#pokemon-tabs-cherubi-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-cherubi-panel-1 { display: block; }
</style>
</details>
