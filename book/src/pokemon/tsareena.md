<details class="pokemon-card-container">
<summary>Tsareena (#133)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-tsareena">
<input type="radio" name="pokemon-tabs-tsareena-group" id="pokemon-tabs-tsareena-tab-0">
<label for="pokemon-tabs-tsareena-tab-0">Bounsweet</label>
<input type="radio" name="pokemon-tabs-tsareena-group" id="pokemon-tabs-tsareena-tab-1">
<label for="pokemon-tabs-tsareena-tab-1">Steenee</label>
<input type="radio" name="pokemon-tabs-tsareena-group" id="pokemon-tabs-tsareena-tab-2" checked>
<label for="pokemon-tabs-tsareena-tab-2">Tsareena</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-tsareena-panel-0">
Types: Grass • Egg Groups: Grass

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Leaf Guard
- Oblivious
- Sweet Veil *(Hidden)*

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
- TM16 - Light Screen
- TM17 - Protect
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM32 - Double Team
- TM33 - Reflect
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM54 - Dazzling Gleam

**Held Item**
Grassy Seed

**Encounter Locations**
- Jusmail Town — Grass (Day) (10%)
- Riverwalk Trail (South) — Grass (Day) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="bounsweet" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">42</span> |
| Attack | <span class="stat-value stat-low">30</span> |
| Defense | <span class="stat-value stat-low">38</span> |
| Sp. Atk | <span class="stat-value stat-low">30</span> |
| Sp. Def | <span class="stat-value stat-low">38</span> |
| Speed | <span class="stat-value stat-low">32</span> |
| Total | <span class="stat-value stat-low">210</span> |

**Level-Up Moves**
- Splash (Lv 1)
- Play Nice (Lv 5)
- Rapid Spin (Lv 9)
- Razor Leaf (Lv 13)
- Synthesis (Lv 15)
- Sweet Scent (Lv 17)
- Magical Leaf (Lv 21)
- Teeter Dance (Lv 25)
- Stomp (Lv 29)
- Aromatic Mist (Lv 33)

**Egg Moves**
- Grass Whistle
- Synthesis
- Play Rough
- Feint
- Charm
- Acupressure

**Tutor Moves**
- Endure
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
<div class="pokemon-tab-panel" id="pokemon-tabs-tsareena-panel-1">
Types: Grass • Egg Groups: Grass

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Leaf Guard
- Oblivious
- Sweet Veil *(Hidden)*

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
- TM16 - Light Screen
- TM17 - Protect
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM32 - Double Team
- TM33 - Reflect
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM54 - Dazzling Gleam

**Held Item**
Grassy Seed

**Evolution Info**
Lv. 18

**Encounter Locations**
- Kipos Town — Grass (Day) (8%)
- Myrrini Island — Grass (Day) (20%)
- Tower of Dioxippus — Grass (Day) (10%)
- Tower of Dioxippus — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="steenee" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">52</span> |
| Attack | <span class="stat-value stat-low">45</span> |
| Defense | <span class="stat-value stat-low">48</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-low">48</span> |
| Speed | <span class="stat-value stat-mid">62</span> |
| Total | <span class="stat-value stat-low">295</span> |

**Level-Up Moves**
- Double Slap (Lv Evo)
- Splash (Lv 1)
- Play Nice (Lv 5)
- Rapid Spin (Lv 9)
- Razor Leaf (Lv 13)
- Synthesis (Lv 15)
- Sweet Scent (Lv 17)
- Magical Leaf (Lv 21)
- Teeter Dance (Lv 25)
- Stomp (Lv 29)
- Aromatic Mist (Lv 33)
- Captivate (Lv 37)
- Aromatherapy (Lv 41)
- Leaf Storm (Lv 45)

**Egg Moves**
- Grass Whistle
- Synthesis
- Play Rough
- Feint
- Charm
- Acupressure

**Tutor Moves**
- Endure
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
<div class="pokemon-tab-panel" id="pokemon-tabs-tsareena-panel-2">
Types: Grass • Egg Groups: Grass

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Leaf Guard
- Queenly Majesty
- Strong Legs *(Hidden)*

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
- TM12 - Taunt
- TM15 - Draining Kiss
- TM16 - Light Screen
- TM17 - Protect
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM32 - Double Team
- TM33 - Reflect
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM54 - Dazzling Gleam

**Held Item**
Grassy Seed

**Evolution Info**
Lv. knows Stomp
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="tsareena" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">72</span> |
| Attack | <span class="stat-value stat-high">120</span> |
| Defense | <span class="stat-value stat-high">98</span> |
| Sp. Atk | <span class="stat-value stat-low">45</span> |
| Sp. Def | <span class="stat-value stat-high">98</span> |
| Speed | <span class="stat-value stat-high">97</span> |
| Total | <span class="stat-value stat-mid">530</span> |

**Level-Up Moves**
- Trop Kick (Lv Evo)
- Punishment (Lv 1)
- Double Slap (Lv 1)
- Splash (Lv 1)
- Swagger (Lv 5)
- Rapid Spin (Lv 9)
- Razor Leaf (Lv 13)
- Synthesis (Lv 15)
- Sweet Scent (Lv 17)
- Magical Leaf (Lv 21)
- Teeter Dance (Lv 25)
- Stomp (Lv 29)
- Aromatic Mist (Lv 33)
- Stomping Tantrum (Lv 35)
- Captivate (Lv 37)
- Aromatherapy (Lv 41)
- High Jump Kick (Lv 43)
- Leaf Storm (Lv 45)
- Thunderous Kick (Lv 49)
- Power Whip (Lv 53)

**Egg Moves**
- Grass Whistle
- Synthesis
- Play Rough
- Feint
- Charm
- Acupressure

**Tutor Moves**
- Endure
- Mega Kick
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
#pokemon-tabs-tsareena-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-tsareena-panel-0 { display: block; }
#pokemon-tabs-tsareena-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-tsareena-panel-1 { display: block; }
#pokemon-tabs-tsareena-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-tsareena-panel-2 { display: block; }
</style>
</details>
