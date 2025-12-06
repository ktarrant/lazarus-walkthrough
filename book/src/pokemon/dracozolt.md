<details class="pokemon-card-container">
<summary>Dracozolt (#408)</summary>
Types: Electric / Dragon • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Volt Absorb
- Strong Jaw
- Sand Stream *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Electric (0.25×)
- Grass (0.5×)
- Flying (0.5×)
- Steel (0.5×)

*Weak to*
- Ice (2×)
- Ground (2×)
- Dragon (2×)
- Fairy (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=dragon-claw">TM02 - Dragon Claw</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=thunder">TM25 - Thunder</a>
- <a href="move-lookup.html?q=earthquake">TM26 - Earthquake</a>
- <a href="move-lookup.html?q=flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=fire-blast">TM38 - Fire Blast</a>
- <a href="move-lookup.html?q=rock-tomb">TM39 - Rock Tomb</a>
- <a href="move-lookup.html?q=aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=bulldoze">TM49 - Bulldoze</a>
- <a href="move-lookup.html?q=thunder-wave">TM58 - Thunder Wave</a>
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="dracozolt" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">90</span> |
| Attack | <span class="stat-value stat-high">100</span> |
| Defense | <span class="stat-value stat-mid">90</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-mid">70</span> |
| Speed | <span class="stat-value stat-mid">75</span> |
| Total | <span class="stat-value stat-mid">505</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=thunder-shock">Thunder Shock</a> (Lv 1)
- <a href="move-lookup.html?q=charge">Charge</a> (Lv 7)
- <a href="move-lookup.html?q=aerial-ace">Aerial Ace</a> (Lv 12)
- <a href="move-lookup.html?q=ancient-power">Ancient Power</a> (Lv 16)
- <a href="move-lookup.html?q=spark">Spark</a> (Lv 20)
- <a href="move-lookup.html?q=pluck">Pluck</a> (Lv 24)
- <a href="move-lookup.html?q=dragon-tail">Dragon Tail</a> (Lv 28)
- <a href="move-lookup.html?q=thunder-fang">Thunder Fang</a> (Lv 33)
- <a href="move-lookup.html?q=stomp">Stomp</a> (Lv 35)
- <a href="move-lookup.html?q=stomping-tantrum">Stomping Tantrum</a> (Lv 38)
- <a href="move-lookup.html?q=slam">Slam</a> (Lv 40)
- <a href="move-lookup.html?q=breaking-swipe">Breaking Swipe</a> (Lv 44)
- <a href="move-lookup.html?q=discharge">Discharge</a> (Lv 48)
- <a href="move-lookup.html?q=bolt-beak">Bolt Beak</a> (Lv 50)
- <a href="move-lookup.html?q=dragon-pulse">Dragon Pulse</a> (Lv 53)
- <a href="move-lookup.html?q=dragon-rush">Dragon Rush</a> (Lv 57)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=mega-kick">Mega Kick</a>
- <a href="move-lookup.html?q=mega-punch">Mega Punch</a>
- <a href="move-lookup.html?q=rock-slide">Rock Slide</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=thunder-punch">Thunder Punch</a>
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
