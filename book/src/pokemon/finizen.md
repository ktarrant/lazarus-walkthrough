<details class="pokemon-card-container">
<summary>Finizen (#162)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-finizen">
<input type="radio" name="pokemon-tabs-finizen-group" id="pokemon-tabs-finizen-tab-0" checked>
<label for="pokemon-tabs-finizen-tab-0">Finizen</label>
<input type="radio" name="pokemon-tabs-finizen-group" id="pokemon-tabs-finizen-tab-1">
<label for="pokemon-tabs-finizen-tab-1">Palafin</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-finizen-panel-0">
Types: Water • Egg Groups: Field / Water 2

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Water Veil
- Water Veil *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm03-water-pulse">TM03 - Water Pulse</a>
- <a href="move-lookup.html?q=tm13-ice-beam">TM13 - Ice Beam</a>
- <a href="move-lookup.html?q=tm14-blizzard">TM14 - Blizzard</a>
- <a href="move-lookup.html?q=tm15-draining-kiss">TM15 - Draining Kiss</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm50-deepwater-curse">TM50 - Deepwater Curse</a>
- <a href="move-lookup.html?q=hm03-surf">HM03 - Surf</a>
- <a href="move-lookup.html?q=hm07-waterfall">HM07 - Waterfall</a>
- <a href="move-lookup.html?q=hm08-dive">HM08 - Dive</a>

**Encounter Locations**
- Davosi Straits — Surfing (60%)
- Erinys Path (East) — Fishing (20%)
- Marmaro Island — Surfing (30%)
- Myrrini Island — Surfing (10%)
- Pollen Road — Surfing (60%)
- Pythios Town — Fishing (20%)
- Pythios Town — Surfing (10%)
- Péntepetal City — Surfing (30%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="finizen" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-low">45</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-mid">75</span> |
| Total | <span class="stat-value stat-mid">335</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=supersonic">Supersonic</a> (Lv 1)
- <a href="move-lookup.html?q=water-gun">Water Gun</a> (Lv 1)
- <a href="move-lookup.html?q=astonish">Astonish</a> (Lv 7)
- <a href="move-lookup.html?q=focus-energy">Focus Energy</a> (Lv 10)
- <a href="move-lookup.html?q=aqua-jet">Aqua Jet</a> (Lv 13)
- <a href="move-lookup.html?q=pluck">Pluck</a> (Lv 15)
- <a href="move-lookup.html?q=double-hit">Double Hit</a> (Lv 17)
- <a href="move-lookup.html?q=dive">Dive</a> (Lv 21)
- <a href="move-lookup.html?q=charm">Charm</a> (Lv 23)
- <a href="move-lookup.html?q=covet">Covet</a> (Lv 26)
- <a href="move-lookup.html?q=acrobatics">Acrobatics</a> (Lv 29)
- <a href="move-lookup.html?q=encore">Encore</a> (Lv 34)
- <a href="move-lookup.html?q=aqua-tail">Aqua Tail</a> (Lv 39)
- <a href="move-lookup.html?q=mist">Mist</a> (Lv 44)
- <a href="move-lookup.html?q=hydro-pump">Hydro Pump</a> (Lv 50)

**Egg Moves**
- <a href="move-lookup.html?q=boomburst">Boomburst</a>
- <a href="move-lookup.html?q=bounce">Bounce</a>
- <a href="move-lookup.html?q=counter">Counter</a>
- <a href="move-lookup.html?q=haze">Haze</a>
- <a href="move-lookup.html?q=tickle">Tickle</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=counter">Counter</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=icy-wind">Icy Wind</a>
- <a href="move-lookup.html?q=psych-up">Psych Up</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-finizen-panel-1">
Types: Water • Egg Groups: Field / Water 2

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Zero to Hero
- Iron Fist *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm03-water-pulse">TM03 - Water Pulse</a>
- <a href="move-lookup.html?q=tm07-whirlpool">TM07 - Whirlpool</a>
- <a href="move-lookup.html?q=tm08-bulk-up">TM08 - Bulk Up</a>
- <a href="move-lookup.html?q=tm12-taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=tm13-ice-beam">TM13 - Ice Beam</a>
- <a href="move-lookup.html?q=tm14-blizzard">TM14 - Blizzard</a>
- <a href="move-lookup.html?q=tm15-draining-kiss">TM15 - Draining Kiss</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm50-deepwater-curse">TM50 - Deepwater Curse</a>
- <a href="move-lookup.html?q=hm03-surf">HM03 - Surf</a>
- <a href="move-lookup.html?q=hm07-waterfall">HM07 - Waterfall</a>
- <a href="move-lookup.html?q=hm08-dive">HM08 - Dive</a>

**Evolution Info**
Lv. 38
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="palafin" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">100</span> |
| Attack | <span class="stat-value stat-mid">80</span> |
| Defense | <span class="stat-value stat-mid">72</span> |
| Sp. Atk | <span class="stat-value stat-mid">53</span> |
| Sp. Def | <span class="stat-value stat-mid">62</span> |
| Speed | <span class="stat-value stat-high">100</span> |
| Total | <span class="stat-value stat-mid">467</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=flip-turn">Flip Turn</a> (Lv Evo)
- <a href="move-lookup.html?q=jet-punch">Jet Punch</a> (Lv 1)
- <a href="move-lookup.html?q=supersonic">Supersonic</a> (Lv 1)
- <a href="move-lookup.html?q=water-gun">Water Gun</a> (Lv 1)
- <a href="move-lookup.html?q=astonish">Astonish</a> (Lv 7)
- <a href="move-lookup.html?q=focus-energy">Focus Energy</a> (Lv 10)
- <a href="move-lookup.html?q=aqua-jet">Aqua Jet</a> (Lv 13)
- <a href="move-lookup.html?q=pluck">Pluck</a> (Lv 15)
- <a href="move-lookup.html?q=double-hit">Double Hit</a> (Lv 17)
- <a href="move-lookup.html?q=dive">Dive</a> (Lv 21)
- <a href="move-lookup.html?q=charm">Charm</a> (Lv 23)
- <a href="move-lookup.html?q=covet">Covet</a> (Lv 26)
- <a href="move-lookup.html?q=acrobatics">Acrobatics</a> (Lv 29)
- <a href="move-lookup.html?q=encore">Encore</a> (Lv 34)
- <a href="move-lookup.html?q=aqua-tail">Aqua Tail</a> (Lv 39)
- <a href="move-lookup.html?q=jet-punch">Jet Punch</a> (Lv 42)
- <a href="move-lookup.html?q=deepwater-curse">Deepwater Curse</a> (Lv 44)
- <a href="move-lookup.html?q=ice-hammer">Ice Hammer</a> (Lv 47)
- <a href="move-lookup.html?q=hydro-pump">Hydro Pump</a> (Lv 50)
- <a href="move-lookup.html?q=meteor-mash">Meteor Mash</a> (Lv 53)
- <a href="move-lookup.html?q=focus-punch">Focus Punch</a> (Lv 55)
- <a href="move-lookup.html?q=wave-crash">Wave Crash</a> (Lv 61)

**Egg Moves**
- <a href="move-lookup.html?q=boomburst">Boomburst</a>
- <a href="move-lookup.html?q=bounce">Bounce</a>
- <a href="move-lookup.html?q=counter">Counter</a>
- <a href="move-lookup.html?q=haze">Haze</a>
- <a href="move-lookup.html?q=tickle">Tickle</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=counter">Counter</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=ice-punch">Ice Punch</a>
- <a href="move-lookup.html?q=icy-wind">Icy Wind</a>
- <a href="move-lookup.html?q=psych-up">Psych Up</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
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
#pokemon-tabs-finizen-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-finizen-panel-0 { display: block; }
#pokemon-tabs-finizen-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-finizen-panel-1 { display: block; }
</style>
</details>
