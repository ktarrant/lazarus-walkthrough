<details class="pokemon-card-container">
<summary>Dhelmise (#417)</summary>
Types: Ghost / Grass • Egg Groups: Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Steelworker

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Fighting (0×)
- Ground (0.5×)

*Weak to*
- Fire (2×)
- Ice (2×)
- Flying (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=whirlpool">TM07 - Whirlpool</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=hex">TM23 - Hex</a>
- <a href="move-lookup.html?q=earthquake">TM26 - Earthquake</a>
- <a href="move-lookup.html?q=shadow-ball">TM30 - Shadow Ball</a>
- <a href="move-lookup.html?q=brick-break">TM31 - Brick Break</a>
- <a href="move-lookup.html?q=double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=bulldoze">TM49 - Bulldoze</a>
- <a href="move-lookup.html?q=surf">HM03 - Surf</a>

**Encounter Locations**
- Davosi Straits — Surfing (5%)
- Palati City — Fishing (5%)
- Pollen Road — Surfing (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="dhelmise" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">80</span> |
| Attack | <span class="stat-value stat-high">131</span> |
| Defense | <span class="stat-value stat-high">120</span> |
| Sp. Atk | <span class="stat-value stat-mid">76</span> |
| Sp. Def | <span class="stat-value stat-high">93</span> |
| Speed | <span class="stat-value stat-low">20</span> |
| Total | <span class="stat-value stat-mid">520</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=switcheroo">Switcheroo</a> (Lv 1)
- <a href="move-lookup.html?q=absorb">Absorb</a> (Lv 1)
- <a href="move-lookup.html?q=growth">Growth</a> (Lv 1)
- <a href="move-lookup.html?q=rapid-spin">Rapid Spin</a> (Lv 1)
- <a href="move-lookup.html?q=astonish">Astonish</a> (Lv 1)
- <a href="move-lookup.html?q=mega-drain">Mega Drain</a> (Lv 5)
- <a href="move-lookup.html?q=wrap">Wrap</a> (Lv 9)
- <a href="move-lookup.html?q=gyro-ball">Gyro Ball</a> (Lv 14)
- <a href="move-lookup.html?q=metal-sound">Metal Sound</a> (Lv 18)
- <a href="move-lookup.html?q=giga-drain">Giga Drain</a> (Lv 23)
- <a href="move-lookup.html?q=razor-shell">Razor Shell</a> (Lv 25)
- <a href="move-lookup.html?q=whirlpool">Whirlpool</a> (Lv 27)
- <a href="move-lookup.html?q=anchor-shot">Anchor Shot</a> (Lv 30)
- <a href="move-lookup.html?q=shadow-ball">Shadow Ball</a> (Lv 34)
- <a href="move-lookup.html?q=flip-turn">Flip Turn</a> (Lv 37)
- <a href="move-lookup.html?q=energy-ball">Energy Ball</a> (Lv 41)
- <a href="move-lookup.html?q=slam">Slam</a> (Lv 43)
- <a href="move-lookup.html?q=heavy-slam">Heavy Slam</a> (Lv 47)
- <a href="move-lookup.html?q=phantom-force">Phantom Force</a> (Lv 50)
- <a href="move-lookup.html?q=power-whip">Power Whip</a> (Lv 55)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=rock-slide">Rock Slide</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
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
</details>
