<details class="pokemon-card-container">
<summary>Charcadet (#302)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-charcadet">
<input type="radio" name="pokemon-tabs-charcadet-group" id="pokemon-tabs-charcadet-tab-0" checked>
<label for="pokemon-tabs-charcadet-tab-0">Charcadet</label>
<input type="radio" name="pokemon-tabs-charcadet-group" id="pokemon-tabs-charcadet-tab-1">
<label for="pokemon-tabs-charcadet-tab-1">Armarouge</label>
<input type="radio" name="pokemon-tabs-charcadet-group" id="pokemon-tabs-charcadet-tab-2">
<label for="pokemon-tabs-charcadet-tab-2">Ceruledge</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-charcadet-panel-0">
Types: Fire • Egg Groups: Human-Like

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Flash Fire
- Flame Body
- Weak Armor *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Bug (0.5×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Ground (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=fire-blast">TM38 - Fire Blast</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=will-o-wisp">TM51 - Will-O-Wisp</a>

**Encounter Locations**
- Sea of Asteri (East) — Grass (Day) (5%)
- Sea of Vulcai — Grass (Day) (10%)
- Sea of Vulcai — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="charcadet" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">40</span> |
| Attack | <span class="stat-value stat-low">50</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-low">50</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-low">35</span> |
| Total | <span class="stat-value stat-low">255</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=leer">Leer</a> (Lv 1)
- <a href="move-lookup.html?q=ember">Ember</a> (Lv 1)
- <a href="move-lookup.html?q=astonish">Astonish</a> (Lv 1)
- <a href="move-lookup.html?q=clear-smog">Clear Smog</a> (Lv 8)
- <a href="move-lookup.html?q=fire-spin">Fire Spin</a> (Lv 12)
- <a href="move-lookup.html?q=will-o-wisp">Will-O-Wisp</a> (Lv 16)
- <a href="move-lookup.html?q=night-shade">Night Shade</a> (Lv 20)
- <a href="move-lookup.html?q=flame-charge">Flame Charge</a> (Lv 24)
- <a href="move-lookup.html?q=incinerate">Incinerate</a> (Lv 28)
- <a href="move-lookup.html?q=lava-plume">Lava Plume</a> (Lv 32)

**Egg Moves**
- <a href="move-lookup.html?q=destiny-bond">Destiny Bond</a>
- <a href="move-lookup.html?q=disable">Disable</a>
- <a href="move-lookup.html?q=spite">Spite</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-charcadet-panel-1">
Types: Fire / Psychic • Egg Groups: Human-Like

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Flash Fire
- Mega Launcher
- Weak Armor *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Fighting (0.5×)
- Psychic (0.5×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Ground (2×)
- Rock (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=calm-mind">TM04 - Calm Mind</a>
- <a href="move-lookup.html?q=psyshock">TM05 - Psyshock</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=psychic">TM29 - Psychic</a>
- <a href="move-lookup.html?q=shadow-ball">TM30 - Shadow Ball</a>
- <a href="move-lookup.html?q=reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=fire-blast">TM38 - Fire Blast</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=will-o-wisp">TM51 - Will-O-Wisp</a>
- <a href="move-lookup.html?q=dark-pulse">TM59 - Dark Pulse</a>

**Evolution Info**
Auspicious Armor

**Encounter Locations**
- Lastlight Road — Grass (Day) (2%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="armarouge" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">85</span> |
| Attack | <span class="stat-value stat-mid">60</span> |
| Defense | <span class="stat-value stat-high">100</span> |
| Sp. Atk | <span class="stat-value stat-high">125</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-mid">75</span> |
| Total | <span class="stat-value stat-mid">525</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=psyshock">Psyshock</a> (Lv Evo)
- <a href="move-lookup.html?q=leer">Leer</a> (Lv 1)
- <a href="move-lookup.html?q=ember">Ember</a> (Lv 1)
- <a href="move-lookup.html?q=mystical-fire">Mystical Fire</a> (Lv 1)
- <a href="move-lookup.html?q=astonish">Astonish</a> (Lv 1)
- <a href="move-lookup.html?q=wide-guard">Wide Guard</a> (Lv 1)
- <a href="move-lookup.html?q=clear-smog">Clear Smog</a> (Lv 8)
- <a href="move-lookup.html?q=fire-spin">Fire Spin</a> (Lv 12)
- <a href="move-lookup.html?q=will-o-wisp">Will-O-Wisp</a> (Lv 16)
- <a href="move-lookup.html?q=night-shade">Night Shade</a> (Lv 20)
- <a href="move-lookup.html?q=flame-charge">Flame Charge</a> (Lv 24)
- <a href="move-lookup.html?q=incinerate">Incinerate</a> (Lv 28)
- <a href="move-lookup.html?q=lava-plume">Lava Plume</a> (Lv 32)
- <a href="move-lookup.html?q=aura-sphere">Aura Sphere</a> (Lv 34)
- <a href="move-lookup.html?q=calm-mind">Calm Mind</a> (Lv 37)
- <a href="move-lookup.html?q=ally-switch">Ally Switch</a> (Lv 39)
- <a href="move-lookup.html?q=flamethrower">Flamethrower</a> (Lv 41)
- <a href="move-lookup.html?q=expanding-force">Expanding Force</a> (Lv 45)
- <a href="move-lookup.html?q=armor-cannon">Armor Cannon</a> (Lv 50)

**Egg Moves**
- <a href="move-lookup.html?q=destiny-bond">Destiny Bond</a>
- <a href="move-lookup.html?q=disable">Disable</a>
- <a href="move-lookup.html?q=spite">Spite</a>

**Tutor Moves**
- <a href="move-lookup.html?q=acid-spray">Acid Spray</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=psych-up">Psych Up</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-charcadet-panel-2">
Types: Fire / Ghost • Egg Groups: Human-Like

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Flash Fire
- Sharpness
- Weak Armor *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fire (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Fighting (0×)
- Poison (0.5×)
- Bug (0.25×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Ground (2×)
- Rock (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=dragon-claw">TM02 - Dragon Claw</a>
- <a href="move-lookup.html?q=bulk-up">TM08 - Bulk Up</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=poison-jab">TM20 - Poison Jab</a>
- <a href="move-lookup.html?q=hex">TM23 - Hex</a>
- <a href="move-lookup.html?q=shadow-ball">TM30 - Shadow Ball</a>
- <a href="move-lookup.html?q=brick-break">TM31 - Brick Break</a>
- <a href="move-lookup.html?q=reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=fire-blast">TM38 - Fire Blast</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=will-o-wisp">TM51 - Will-O-Wisp</a>

**Evolution Info**
Malicious Armor

**Encounter Locations**
- Lastlight Road — Grass (Night) (2%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="ceruledge" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">75</span> |
| Attack | <span class="stat-value stat-high">125</span> |
| Defense | <span class="stat-value stat-mid">80</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-high">100</span> |
| Speed | <span class="stat-value stat-mid">85</span> |
| Total | <span class="stat-value stat-mid">525</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=shadow-claw">Shadow Claw</a> (Lv Evo)
- <a href="move-lookup.html?q=night-slash">Night Slash</a> (Lv 1)
- <a href="move-lookup.html?q=shadow-sneak">Shadow Sneak</a> (Lv 1)
- <a href="move-lookup.html?q=quick-guard">Quick Guard</a> (Lv 1)
- <a href="move-lookup.html?q=solar-blade">Solar Blade</a> (Lv 1)
- <a href="move-lookup.html?q=leer">Leer</a> (Lv 1)
- <a href="move-lookup.html?q=ember">Ember</a> (Lv 1)
- <a href="move-lookup.html?q=astonish">Astonish</a> (Lv 1)
- <a href="move-lookup.html?q=clear-smog">Clear Smog</a> (Lv 8)
- <a href="move-lookup.html?q=fire-spin">Fire Spin</a> (Lv 12)
- <a href="move-lookup.html?q=will-o-wisp">Will-O-Wisp</a> (Lv 16)
- <a href="move-lookup.html?q=night-shade">Night Shade</a> (Lv 20)
- <a href="move-lookup.html?q=flame-charge">Flame Charge</a> (Lv 24)
- <a href="move-lookup.html?q=incinerate">Incinerate</a> (Lv 28)
- <a href="move-lookup.html?q=lava-plume">Lava Plume</a> (Lv 32)
- <a href="move-lookup.html?q=sacred-sword">Sacred Sword</a> (Lv 34)
- <a href="move-lookup.html?q=swords-dance">Swords Dance</a> (Lv 37)
- <a href="move-lookup.html?q=ally-switch">Ally Switch</a> (Lv 39)
- <a href="move-lookup.html?q=bitter-blade">Bitter Blade</a> (Lv 41)
- <a href="move-lookup.html?q=psycho-cut">Psycho Cut</a> (Lv 45)
- <a href="move-lookup.html?q=flare-blitz">Flare Blitz</a> (Lv 50)

**Egg Moves**
- <a href="move-lookup.html?q=destiny-bond">Destiny Bond</a>
- <a href="move-lookup.html?q=disable">Disable</a>
- <a href="move-lookup.html?q=spite">Spite</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=psych-up">Psych Up</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=swords-dance">Swords Dance</a>
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
#pokemon-tabs-charcadet-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-charcadet-panel-0 { display: block; }
#pokemon-tabs-charcadet-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-charcadet-panel-1 { display: block; }
#pokemon-tabs-charcadet-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-charcadet-panel-2 { display: block; }
</style>
</details>
