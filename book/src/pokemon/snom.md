<details class="pokemon-card-container">
<summary>Snom (#055)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-snom">
<input type="radio" name="pokemon-tabs-snom-group" id="pokemon-tabs-snom-tab-0" checked>
<label for="pokemon-tabs-snom-tab-0">Snom</label>
<input type="radio" name="pokemon-tabs-snom-group" id="pokemon-tabs-snom-tab-1">
<label for="pokemon-tabs-snom-tab-1">Frosmoth</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-snom-panel-0">
Types: Ice / Bug • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Shield Dust
- Ice Scales *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Ice (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (4×)
- Flying (2×)
- Rock (4×)
- Steel (2×)

**TM/HM Moves**
- TM17 - Protect
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract

**Held Item**
Snowball

**Encounter Locations**
- Froslass Cavern BF2 — Grass (Day) (5%)
- Pythios Cemetery — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="snom" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">30</span> |
| Attack | <span class="stat-value stat-low">25</span> |
| Defense | <span class="stat-value stat-low">35</span> |
| Sp. Atk | <span class="stat-value stat-low">45</span> |
| Sp. Def | <span class="stat-value stat-low">30</span> |
| Speed | <span class="stat-value stat-low">20</span> |
| Total | <span class="stat-value stat-low">185</span> |

**Level-Up Moves**
- Powder Snow (Lv 1)
- Struggle Bug (Lv 1)
- Sticky Web (Lv 15)

**Egg Moves**
- Fairy Wind
- Mirror Coat
- Bug Bite

**Tutor Moves**
- Endure
- Icy Wind
- Sleep Talk
- Snore
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
<div class="pokemon-tab-panel" id="pokemon-tabs-snom-panel-1">
Types: Ice / Bug • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Shield Dust
- Ice Scales *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Ice (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (4×)
- Flying (2×)
- Rock (4×)
- Steel (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM13 - Ice Beam
- TM14 - Blizzard
- TM16 - Light Screen
- TM17 - Protect
- TM19 - Giga Drain
- TM33 - Reflect
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM54 - Dazzling Gleam

**Evolution Info**
Ice Stone
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="frosmoth" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">90</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-mid">60</span> |
| Sp. Atk | <span class="stat-value stat-high">125</span> |
| Sp. Def | <span class="stat-value stat-mid">90</span> |
| Speed | <span class="stat-value stat-mid">75</span> |
| Total | <span class="stat-value stat-mid">505</span> |

**Level-Up Moves**
- Icy Wind (Lv Evo)
- Powder Snow (Lv 1)
- Struggle Bug (Lv 1)
- Helping Hand (Lv 1)
- Attract (Lv 1)
- Sticky Web (Lv 1)
- Stun Spore (Lv 4)
- Infestation (Lv 8)
- Mist (Lv 12)
- Defog (Lv 16)
- Mega Drain (Lv 20)
- Feather Dance (Lv 22)
- Aurora Beam (Lv 24)
- Hail (Lv 28)
- Freeze-Dry (Lv 30)
- Bug Buzz (Lv 32)
- Aurora Veil (Lv 35)
- Quiver Dance (Lv 38)
- Blizzard (Lv 40)
- Giga Drain (Lv 42)
- Tailwind (Lv 45)
- Wide Guard (Lv 48)
- Quiver Dance (Lv 52)

**Egg Moves**
- Fairy Wind
- Mirror Coat
- Bug Bite

**Tutor Moves**
- Endure
- Icy Wind
- Sleep Talk
- Snore
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
#pokemon-tabs-snom-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-snom-panel-0 { display: block; }
#pokemon-tabs-snom-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-snom-panel-1 { display: block; }
</style>
</details>
