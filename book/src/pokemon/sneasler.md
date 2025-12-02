<details class="pokemon-card-container">
<summary>Sneasler (#202)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-sneasler">
<input type="radio" name="pokemon-tabs-sneasler-group" id="pokemon-tabs-sneasler-tab-0">
<label for="pokemon-tabs-sneasler-tab-0">Hisuian Sneasel</label>
<input type="radio" name="pokemon-tabs-sneasler-group" id="pokemon-tabs-sneasler-tab-1" checked>
<label for="pokemon-tabs-sneasler-tab-1">Sneasler</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-sneasler-panel-0">
Types: Poison / Fighting • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Inner Focus
- Keen Eye
- Pickpocket *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Fighting (0.5×)
- Poison (0.5×)
- Bug (0.25×)
- Rock (0.5×)
- Dark (0.5×)

*Weak to*
- Ground (2×)
- Flying (2×)
- Psychic (4×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM06 - Toxic
- TM08 - Bulk Up
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM20 - Poison Jab
- TM28 - Dig
- TM30 - Shadow Ball
- TM31 - Brick Break
- TM36 - Sludge Bomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM43 - Poison Fang
- TM44 - Rest
- TM46 - Thief
- TM53 - Power-Up Punch
- TM55 - Snarl
- HM06 - Rock Smash

**Held Item**
Quick Claw

**Encounter Locations**
- Froslass Cavern F1 — Grass (Day) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="hisuian-sneasel" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">55</span> |
| Attack | <span class="stat-value stat-high">95</span> |
| Defense | <span class="stat-value stat-mid">55</span> |
| Sp. Atk | <span class="stat-value stat-low">35</span> |
| Sp. Def | <span class="stat-value stat-mid">75</span> |
| Speed | <span class="stat-value stat-high">115</span> |
| Total | <span class="stat-value stat-mid">430</span> |

**Level-Up Moves**
- Scratch (Lv 1)
- Leer (Lv 1)
- Taunt (Lv 1)
- Quick Attack (Lv 8)
- Poison Sting (Lv 10)
- Rock Smash (Lv 14)
- Fury Swipes (Lv 16)
- Ice Shard (Lv 19)
- Agility (Lv 20)
- Metal Claw (Lv 22)
- Hone Claws (Lv 25)
- Poison Jab (Lv 27)
- Knock Off (Lv 28)
- Screech (Lv 32)
- Slash (Lv 34)
- Swords Dance (Lv 37)
- Snatch (Lv 40)
- Dire Claw (Lv 44)
- Close Combat (Lv 47)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Acid Spray
- Counter
- Endure
- Sleep Talk
- Swift
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
<div class="pokemon-tab-panel" id="pokemon-tabs-sneasler-panel-1">
Types: Fighting / Poison • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Pressure
- Unburden
- Poison Touch *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Fighting (0.5×)
- Poison (0.5×)
- Bug (0.25×)
- Rock (0.5×)
- Dark (0.5×)

*Weak to*
- Ground (2×)
- Flying (2×)
- Psychic (4×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM06 - Toxic
- TM08 - Bulk Up
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM20 - Poison Jab
- TM28 - Dig
- TM30 - Shadow Ball
- TM31 - Brick Break
- TM36 - Sludge Bomb
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM43 - Poison Fang
- TM44 - Rest
- TM46 - Thief
- TM53 - Power-Up Punch
- TM55 - Snarl
- HM06 - Rock Smash

**Evolution Info**
Razor Claw
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="sneasler" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">80</span> |
| Attack | <span class="stat-value stat-high">130</span> |
| Defense | <span class="stat-value stat-mid">60</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-high">120</span> |
| Total | <span class="stat-value stat-mid">510</span> |

**Level-Up Moves**
- Scratch (Lv 1)
- Leer (Lv 1)
- Taunt (Lv 1)
- Quick Attack (Lv 8)
- Poison Sting (Lv 10)
- Rock Smash (Lv 14)
- Fury Swipes (Lv 16)
- Ice Shard (Lv 19)
- Agility (Lv 20)
- Metal Claw (Lv 22)
- Hone Claws (Lv 25)
- Poison Jab (Lv 27)
- Knock Off (Lv 28)
- Screech (Lv 32)
- Slash (Lv 34)
- Swords Dance (Lv 37)
- Snatch (Lv 40)
- Dire Claw (Lv 44)
- Close Combat (Lv 47)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Acid Spray
- Counter
- Endure
- Fire Punch
- Rock Slide
- Sleep Talk
- Swift
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
#pokemon-tabs-sneasler-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-sneasler-panel-0 { display: block; }
#pokemon-tabs-sneasler-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-sneasler-panel-1 { display: block; }
</style>
</details>
