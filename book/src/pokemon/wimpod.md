<details class="pokemon-card-container">
<summary>Wimpod (#246)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-wimpod">
<input type="radio" name="pokemon-tabs-wimpod-group" id="pokemon-tabs-wimpod-tab-0" checked>
<label for="pokemon-tabs-wimpod-tab-0">Wimpod</label>
<input type="radio" name="pokemon-tabs-wimpod-group" id="pokemon-tabs-wimpod-tab-1">
<label for="pokemon-tabs-wimpod-tab-1">Golisopod</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-wimpod-panel-0">
Types: Bug / Water • Egg Groups: Bug / Water 3

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Wimp Out
- Defeatist *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Ice (0.5×)
- Fighting (0.5×)
- Ground (0.5×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Flying (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=scald">TM56 - Scald</a>
- <a href="move-lookup.html?q=surf">HM03 - Surf</a>
- <a href="move-lookup.html?q=waterfall">HM07 - Waterfall</a>

**Encounter Locations**
- Acrisia City — Fishing (30%)
- Acrisia City — Surfing (60%)
- Acrisia Mountains — Fishing (30%)
- Acrisia Mountains — Surfing (60%)
- Kalami City — Grass (Day) (10%)
- Kalami City — Grass (Night) (10%)
- Pythios Cemetery — Surfing (60%)
- Wanderer's Woods (North) — Surfing (60%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="wimpod" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">25</span> |
| Attack | <span class="stat-value stat-low">35</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-low">20</span> |
| Sp. Def | <span class="stat-value stat-low">30</span> |
| Speed | <span class="stat-value stat-mid">80</span> |
| Total | <span class="stat-value stat-low">230</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=struggle-bug">Struggle Bug</a> (Lv 1)
- <a href="move-lookup.html?q=sand-attack">Sand Attack</a> (Lv 1)
- <a href="move-lookup.html?q=bug-bite">Bug Bite</a> (Lv 15)
- <a href="move-lookup.html?q=razor-shell">Razor Shell</a> (Lv 25)

**Egg Moves**
- <a href="move-lookup.html?q=spikes">Spikes</a>
- <a href="move-lookup.html?q=metal-claw">Metal Claw</a>
- <a href="move-lookup.html?q=wide-guard">Wide Guard</a>
- <a href="move-lookup.html?q=harden">Harden</a>
- <a href="move-lookup.html?q=aqua-jet">Aqua Jet</a>

**Tutor Moves**
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-wimpod-panel-1">
Types: Bug / Water • Egg Groups: Bug / Water 3

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Emergency Exit
- Anger Shell *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Ice (0.5×)
- Fighting (0.5×)
- Ground (0.5×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Flying (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=water-pulse">TM03 - Water Pulse</a>
- <a href="move-lookup.html?q=toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=bulk-up">TM08 - Bulk Up</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=ice-beam">TM13 - Ice Beam</a>
- <a href="move-lookup.html?q=blizzard">TM14 - Blizzard</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=poison-jab">TM20 - Poison Jab</a>
- <a href="move-lookup.html?q=brick-break">TM31 - Brick Break</a>
- <a href="move-lookup.html?q=double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=sludge-bomb">TM36 - Sludge Bomb</a>
- <a href="move-lookup.html?q=rock-tomb">TM39 - Rock Tomb</a>
- <a href="move-lookup.html?q=aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=frost-breath">TM52 - Frost Breath</a>
- <a href="move-lookup.html?q=snarl">TM55 - Snarl</a>
- <a href="move-lookup.html?q=scald">TM56 - Scald</a>
- <a href="move-lookup.html?q=dark-pulse">TM59 - Dark Pulse</a>
- <a href="move-lookup.html?q=surf">HM03 - Surf</a>
- <a href="move-lookup.html?q=rock-smash">HM06 - Rock Smash</a>
- <a href="move-lookup.html?q=waterfall">HM07 - Waterfall</a>
- <a href="move-lookup.html?q=dive">HM08 - Dive</a>

**Evolution Info**
Lv. 30
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="golisopod" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">75</span> |
| Attack | <span class="stat-value stat-high">125</span> |
| Defense | <span class="stat-value stat-high">140</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-mid">90</span> |
| Speed | <span class="stat-value stat-low">40</span> |
| Total | <span class="stat-value stat-mid">530</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=first-impression">First Impression</a> (Lv Evo)
- <a href="move-lookup.html?q=struggle-bug">Struggle Bug</a> (Lv 1)
- <a href="move-lookup.html?q=sand-attack">Sand Attack</a> (Lv 1)
- <a href="move-lookup.html?q=fury-cutter">Fury Cutter</a> (Lv 4)
- <a href="move-lookup.html?q=rock-smash">Rock Smash</a> (Lv 7)
- <a href="move-lookup.html?q=bug-bite">Bug Bite</a> (Lv 10)
- <a href="move-lookup.html?q=spite">Spite</a> (Lv 13)
- <a href="move-lookup.html?q=swords-dance">Swords Dance</a> (Lv 16)
- <a href="move-lookup.html?q=slash">Slash</a> (Lv 21)
- <a href="move-lookup.html?q=razor-shell">Razor Shell</a> (Lv 26)
- <a href="move-lookup.html?q=sucker-punch">Sucker Punch</a> (Lv 31)
- <a href="move-lookup.html?q=pounce">Pounce</a> (Lv 34)
- <a href="move-lookup.html?q=iron-defense">Iron Defense</a> (Lv 36)
- <a href="move-lookup.html?q=flip-turn">Flip Turn</a> (Lv 39)
- <a href="move-lookup.html?q=pin-missile">Pin Missile</a> (Lv 41)
- <a href="move-lookup.html?q=spiky-shield">Spiky Shield</a> (Lv 45)
- <a href="move-lookup.html?q=liquidation">Liquidation</a> (Lv 48)

**Egg Moves**
- <a href="move-lookup.html?q=spikes">Spikes</a>
- <a href="move-lookup.html?q=metal-claw">Metal Claw</a>
- <a href="move-lookup.html?q=wide-guard">Wide Guard</a>
- <a href="move-lookup.html?q=harden">Harden</a>
- <a href="move-lookup.html?q=aqua-jet">Aqua Jet</a>

**Tutor Moves**
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=fury-cutter">Fury Cutter</a>
- <a href="move-lookup.html?q=icy-wind">Icy Wind</a>
- <a href="move-lookup.html?q=psych-up">Psych Up</a>
- <a href="move-lookup.html?q=rock-slide">Rock Slide</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
- <a href="move-lookup.html?q=swift">Swift</a>
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
#pokemon-tabs-wimpod-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-wimpod-panel-0 { display: block; }
#pokemon-tabs-wimpod-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-wimpod-panel-1 { display: block; }
</style>
</details>
