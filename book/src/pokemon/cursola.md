<details class="pokemon-card-container">
<summary>Cursola (#255)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-cursola">
<input type="radio" name="pokemon-tabs-cursola-group" id="pokemon-tabs-cursola-tab-0">
<label for="pokemon-tabs-cursola-tab-0">Galarian Corsola</label>
<input type="radio" name="pokemon-tabs-cursola-group" id="pokemon-tabs-cursola-tab-1" checked>
<label for="pokemon-tabs-cursola-tab-1">Cursola</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-cursola-panel-0">
Types: Ghost / Rock • Egg Groups: Water 1 / Water 3

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Weak Armor
- Cursed Body *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fire (0.5×)
- Fighting (0×)
- Poison (0.25×)
- Flying (0.5×)
- Bug (0.5×)

*Weak to*
- Water (2×)
- Grass (2×)
- Ground (2×)
- Ghost (2×)
- Dark (2×)
- Steel (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM04 - Calm Mind
- TM07 - Whirlpool
- TM11 - Sunny Day
- TM13 - Ice Beam
- TM14 - Blizzard
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM23 - Hex
- TM26 - Earthquake
- TM28 - Dig
- TM29 - Psychic
- TM30 - Shadow Ball
- TM33 - Reflect
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM51 - Will-O-Wisp
- TM56 - Scald
- HM03 - Surf

**Encounter Locations**
- Kipos Town — Surfing (5%)
- Marmaro Island — Surfing (5%)
- Sofos City — Surfing (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="galarian-corsola" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-mid">55</span> |
| Defense | <span class="stat-value stat-high">100</span> |
| Sp. Atk | <span class="stat-value stat-mid">65</span> |
| Sp. Def | <span class="stat-value stat-high">100</span> |
| Speed | <span class="stat-value stat-low">30</span> |
| Total | <span class="stat-value stat-mid">410</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Harden (Lv 1)
- Astonish (Lv 5)
- Disable (Lv 10)
- Spite (Lv 14)
- Bubble Beam (Lv 18)
- Ancient Power (Lv 20)
- Hex (Lv 23)
- Rock Blast (Lv 27)
- Curse (Lv 30)
- Strength Sap (Lv 33)
- Shadow Ball (Lv 37)
- Power Gem (Lv 40)
- Night Shade (Lv 43)
- Infernal Parade (Lv 46)
- Grudge (Lv 50)
- Mirror Coat (Lv 55)

**Egg Moves**
- Haze
- Confuse Ray
- Nature Power
- Destiny Bond
- Water Pulse
- Head Smash

**Tutor Moves**
- Body Slam
- Endure
- Icy Wind
- Rollout
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
<div class="pokemon-tab-panel" id="pokemon-tabs-cursola-panel-1">
Types: Ghost / Rock • Egg Groups: Water 1 / Water 3

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Weak Armor
- Perish Body *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fire (0.5×)
- Fighting (0×)
- Poison (0.25×)
- Flying (0.5×)
- Bug (0.5×)

*Weak to*
- Water (2×)
- Grass (2×)
- Ground (2×)
- Ghost (2×)
- Dark (2×)
- Steel (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM04 - Calm Mind
- TM07 - Whirlpool
- TM11 - Sunny Day
- TM13 - Ice Beam
- TM14 - Blizzard
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM23 - Hex
- TM26 - Earthquake
- TM28 - Dig
- TM29 - Psychic
- TM30 - Shadow Ball
- TM33 - Reflect
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM51 - Will-O-Wisp
- TM56 - Scald
- HM03 - Surf

**Evolution Info**
Lv. 34
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="cursola" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-high">95</span> |
| Defense | <span class="stat-value stat-low">50</span> |
| Sp. Atk | <span class="stat-value stat-high">145</span> |
| Sp. Def | <span class="stat-value stat-high">130</span> |
| Speed | <span class="stat-value stat-low">20</span> |
| Total | <span class="stat-value stat-mid">510</span> |

**Level-Up Moves**
- Perish Song (Lv Evo)
- Tackle (Lv 1)
- Harden (Lv 1)
- Astonish (Lv 5)
- Disable (Lv 10)
- Spite (Lv 14)
- Bubble Beam (Lv 18)
- Ancient Power (Lv 20)
- Hex (Lv 23)
- Rock Blast (Lv 27)
- Curse (Lv 30)
- Strength Sap (Lv 33)
- Shadow Ball (Lv 37)
- Power Gem (Lv 40)
- Night Shade (Lv 43)
- Infernal Parade (Lv 46)
- Grudge (Lv 50)
- Mirror Coat (Lv 55)

**Egg Moves**
- Haze
- Confuse Ray
- Nature Power
- Destiny Bond
- Water Pulse
- Head Smash

**Tutor Moves**
- Body Slam
- Endure
- Icy Wind
- Rollout
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
</div>
</div>
<style>
#pokemon-tabs-cursola-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-cursola-panel-0 { display: block; }
#pokemon-tabs-cursola-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-cursola-panel-1 { display: block; }
</style>
</details>
