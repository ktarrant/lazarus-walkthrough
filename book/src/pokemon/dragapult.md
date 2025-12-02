<details class="pokemon-card-container">
<summary>Dragapult (#271)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-dragapult">
<input type="radio" name="pokemon-tabs-dragapult-group" id="pokemon-tabs-dragapult-tab-0">
<label for="pokemon-tabs-dragapult-tab-0">Dreepy</label>
<input type="radio" name="pokemon-tabs-dragapult-group" id="pokemon-tabs-dragapult-tab-1">
<label for="pokemon-tabs-dragapult-tab-1">Drakloak</label>
<input type="radio" name="pokemon-tabs-dragapult-group" id="pokemon-tabs-dragapult-tab-2" checked>
<label for="pokemon-tabs-dragapult-tab-2">Dragapult</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-dragapult-panel-0">
Types: Dragon / Ghost • Egg Groups: Amorphous / Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Clear Body
- Infiltrator
- Cursed Body *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fire (0.5×)
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Fighting (0×)
- Poison (0.5×)
- Bug (0.5×)

*Weak to*
- Ice (2×)
- Ghost (2×)
- Dragon (2×)
- Dark (2×)
- Fairy (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm32-double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm45-attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=tm58-thunder-wave">TM58 - Thunder Wave</a>

**Encounter Locations**
- Fresco Isles — Grass (Night) (5%)
- Riverwalk Trail (South) — Grass (Night) (2%)
- Sea of Asteri (West) — Grass (Day) (2%)
- Sea of Asteri (West) — Grass (Night) (2%)
- Tower of Dioxippus — Grass (Night) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="dreepy" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">28</span> |
| Attack | <span class="stat-value stat-mid">60</span> |
| Defense | <span class="stat-value stat-low">30</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-low">30</span> |
| Speed | <span class="stat-value stat-mid">82</span> |
| Total | <span class="stat-value stat-low">270</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=astonish">Astonish</a> (Lv 1)
- <a href="move-lookup.html?q=infestation">Infestation</a> (Lv 1)
- <a href="move-lookup.html?q=quick-attack">Quick Attack</a> (Lv 1)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 1)
- <a href="move-lookup.html?q=water-gun">Water Gun</a> (Lv 10)
- <a href="move-lookup.html?q=flail">Flail</a> (Lv 17)
- <a href="move-lookup.html?q=hex">Hex</a> (Lv 24)

**Egg Moves**
- <a href="move-lookup.html?q=curse">Curse</a>
- <a href="move-lookup.html?q=grudge">Grudge</a>
- <a href="move-lookup.html?q=confuse-ray">Confuse Ray</a>
- <a href="move-lookup.html?q=double-team">Double Team</a>
- <a href="move-lookup.html?q=disable">Disable</a>
- <a href="move-lookup.html?q=dragon-tail">Dragon Tail</a>
- <a href="move-lookup.html?q=sucker-punch">Sucker Punch</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swift">Swift</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-dragapult-panel-1">
Types: Dragon / Ghost • Egg Groups: Amorphous / Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Clear Body
- Infiltrator
- Cursed Body *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fire (0.5×)
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Fighting (0×)
- Poison (0.5×)
- Bug (0.5×)

*Weak to*
- Ice (2×)
- Ghost (2×)
- Dragon (2×)
- Dark (2×)
- Fairy (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm11-sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=tm16-light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm23-hex">TM23 - Hex</a>
- <a href="move-lookup.html?q=tm24-thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=tm25-thunder">TM25 - Thunder</a>
- <a href="move-lookup.html?q=tm30-shadow-ball">TM30 - Shadow Ball</a>
- <a href="move-lookup.html?q=tm32-double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=tm33-reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=tm35-flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=tm38-fire-blast">TM38 - Fire Blast</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm45-attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=tm46-thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=tm47-steel-wing">TM47 - Steel Wing</a>
- <a href="move-lookup.html?q=tm51-will-o-wisp">TM51 - Will-O-Wisp</a>
- <a href="move-lookup.html?q=tm56-scald">TM56 - Scald</a>
- <a href="move-lookup.html?q=tm58-thunder-wave">TM58 - Thunder Wave</a>
- <a href="move-lookup.html?q=tm60-dragon-dance">TM60 - Dragon Dance</a>
- <a href="move-lookup.html?q=hm03-surf">HM03 - Surf</a>
- <a href="move-lookup.html?q=hm08-dive">HM08 - Dive</a>

**Evolution Info**
Lv. 28

**Encounter Locations**
- Tower of Dioxippus — Grass (Night) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="drakloak" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">68</span> |
| Attack | <span class="stat-value stat-mid">90</span> |
| Defense | <span class="stat-value stat-low">50</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-high">102</span> |
| Total | <span class="stat-value stat-mid">420</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=dragon-breath">Dragon Breath</a> (Lv Evo)
- <a href="move-lookup.html?q=astonish">Astonish</a> (Lv 1)
- <a href="move-lookup.html?q=infestation">Infestation</a> (Lv 1)
- <a href="move-lookup.html?q=quick-attack">Quick Attack</a> (Lv 1)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 1)
- <a href="move-lookup.html?q=lock-on">Lock-On</a> (Lv 6)
- <a href="move-lookup.html?q=assurance">Assurance</a> (Lv 12)
- <a href="move-lookup.html?q=hex">Hex</a> (Lv 18)
- <a href="move-lookup.html?q=agility">Agility</a> (Lv 24)
- <a href="move-lookup.html?q=double-hit">Double Hit</a> (Lv 30)
- <a href="move-lookup.html?q=u-turn">U-Turn</a> (Lv 36)
- <a href="move-lookup.html?q=dragon-pulse">Dragon Pulse</a> (Lv 38)
- <a href="move-lookup.html?q=dragon-dance">Dragon Dance</a> (Lv 44)
- <a href="move-lookup.html?q=phantom-force">Phantom Force</a> (Lv 48)
- <a href="move-lookup.html?q=body-slam">Body Slam</a> (Lv 54)
- <a href="move-lookup.html?q=dragon-darts">Dragon Darts</a> (Lv 60)
- <a href="move-lookup.html?q=dragon-rush">Dragon Rush</a> (Lv 61)
- <a href="move-lookup.html?q=double-edge">Double-Edge</a> (Lv 66)
- <a href="move-lookup.html?q=last-resort">Last Resort</a> (Lv 72)

**Egg Moves**
- <a href="move-lookup.html?q=curse">Curse</a>
- <a href="move-lookup.html?q=grudge">Grudge</a>
- <a href="move-lookup.html?q=confuse-ray">Confuse Ray</a>
- <a href="move-lookup.html?q=double-team">Double Team</a>
- <a href="move-lookup.html?q=disable">Disable</a>
- <a href="move-lookup.html?q=dragon-tail">Dragon Tail</a>
- <a href="move-lookup.html?q=sucker-punch">Sucker Punch</a>

**Tutor Moves**
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swift">Swift</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-dragapult-panel-2">
Types: Dragon / Ghost • Egg Groups: Amorphous / Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Clear Body
- Infiltrator
- Cursed Body *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fire (0.5×)
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Fighting (0×)
- Poison (0.5×)
- Bug (0.5×)

*Weak to*
- Ice (2×)
- Ghost (2×)
- Dragon (2×)
- Dark (2×)
- Fairy (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm02-dragon-claw">TM02 - Dragon Claw</a>
- <a href="move-lookup.html?q=tm11-sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=tm16-light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm22-solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=tm23-hex">TM23 - Hex</a>
- <a href="move-lookup.html?q=tm24-thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=tm25-thunder">TM25 - Thunder</a>
- <a href="move-lookup.html?q=tm30-shadow-ball">TM30 - Shadow Ball</a>
- <a href="move-lookup.html?q=tm32-double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=tm33-reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=tm35-flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=tm38-fire-blast">TM38 - Fire Blast</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm45-attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=tm46-thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=tm47-steel-wing">TM47 - Steel Wing</a>
- <a href="move-lookup.html?q=tm51-will-o-wisp">TM51 - Will-O-Wisp</a>
- <a href="move-lookup.html?q=tm56-scald">TM56 - Scald</a>
- <a href="move-lookup.html?q=tm58-thunder-wave">TM58 - Thunder Wave</a>
- <a href="move-lookup.html?q=tm60-dragon-dance">TM60 - Dragon Dance</a>
- <a href="move-lookup.html?q=hm02-fly">HM02 - Fly</a>
- <a href="move-lookup.html?q=hm03-surf">HM03 - Surf</a>
- <a href="move-lookup.html?q=hm08-dive">HM08 - Dive</a>

**Evolution Info**
Lv. 60
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="dragapult" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">88</span> |
| Attack | <span class="stat-value stat-high">120</span> |
| Defense | <span class="stat-value stat-mid">75</span> |
| Sp. Atk | <span class="stat-value stat-high">100</span> |
| Sp. Def | <span class="stat-value stat-mid">75</span> |
| Speed | <span class="stat-value stat-high">142</span> |
| Total | <span class="stat-value stat-high">600</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=dragon-darts">Dragon Darts</a> (Lv Evo)
- <a href="move-lookup.html?q=dragon-breath">Dragon Breath</a> (Lv 1)
- <a href="move-lookup.html?q=sucker-punch">Sucker Punch</a> (Lv 1)
- <a href="move-lookup.html?q=astonish">Astonish</a> (Lv 1)
- <a href="move-lookup.html?q=infestation">Infestation</a> (Lv 1)
- <a href="move-lookup.html?q=quick-attack">Quick Attack</a> (Lv 1)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 1)
- <a href="move-lookup.html?q=lock-on">Lock-On</a> (Lv 6)
- <a href="move-lookup.html?q=assurance">Assurance</a> (Lv 12)
- <a href="move-lookup.html?q=hex">Hex</a> (Lv 18)
- <a href="move-lookup.html?q=agility">Agility</a> (Lv 24)
- <a href="move-lookup.html?q=double-hit">Double Hit</a> (Lv 30)
- <a href="move-lookup.html?q=u-turn">U-Turn</a> (Lv 36)
- <a href="move-lookup.html?q=dragon-pulse">Dragon Pulse</a> (Lv 38)
- <a href="move-lookup.html?q=dragon-dance">Dragon Dance</a> (Lv 44)
- <a href="move-lookup.html?q=phantom-force">Phantom Force</a> (Lv 48)
- <a href="move-lookup.html?q=body-slam">Body Slam</a> (Lv 54)
- <a href="move-lookup.html?q=dragon-darts">Dragon Darts</a> (Lv 60)
- <a href="move-lookup.html?q=dragon-rush">Dragon Rush</a> (Lv 61)
- <a href="move-lookup.html?q=double-edge">Double-Edge</a> (Lv 66)
- <a href="move-lookup.html?q=last-resort">Last Resort</a> (Lv 72)

**Egg Moves**
- <a href="move-lookup.html?q=curse">Curse</a>
- <a href="move-lookup.html?q=grudge">Grudge</a>
- <a href="move-lookup.html?q=confuse-ray">Confuse Ray</a>
- <a href="move-lookup.html?q=double-team">Double Team</a>
- <a href="move-lookup.html?q=disable">Disable</a>
- <a href="move-lookup.html?q=dragon-tail">Dragon Tail</a>
- <a href="move-lookup.html?q=sucker-punch">Sucker Punch</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swift">Swift</a>
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
#pokemon-tabs-dragapult-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-dragapult-panel-0 { display: block; }
#pokemon-tabs-dragapult-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-dragapult-panel-1 { display: block; }
#pokemon-tabs-dragapult-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-dragapult-panel-2 { display: block; }
</style>
</details>
