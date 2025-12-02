<details class="pokemon-card-container">
<summary>Dracovish (#410)</summary>
Types: Water / Dragon • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Water Absorb
- Strong Jaw
- Drizzle *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.25×)
- Water (0.25×)
- Steel (0.5×)

*Weak to*
- Dragon (2×)
- Fairy (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm07-whirlpool">TM07 - Whirlpool</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm26-earthquake">TM26 - Earthquake</a>
- <a href="move-lookup.html?q=tm39-rock-tomb">TM39 - Rock Tomb</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm49-bulldoze">TM49 - Bulldoze</a>
- <a href="move-lookup.html?q=tm56-scald">TM56 - Scald</a>
- <a href="move-lookup.html?q=hm03-surf">HM03 - Surf</a>
- <a href="move-lookup.html?q=hm07-waterfall">HM07 - Waterfall</a>
- <a href="move-lookup.html?q=hm08-dive">HM08 - Dive</a>
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="dracovish" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">90</span> |
| Attack | <span class="stat-value stat-mid">90</span> |
| Defense | <span class="stat-value stat-high">100</span> |
| Sp. Atk | <span class="stat-value stat-mid">70</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-mid">75</span> |
| Total | <span class="stat-value stat-mid">505</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=water-gun">Water Gun</a> (Lv 1)
- <a href="move-lookup.html?q=protect">Protect</a> (Lv 7)
- <a href="move-lookup.html?q=brutal-swing">Brutal Swing</a> (Lv 12)
- <a href="move-lookup.html?q=ancient-power">Ancient Power</a> (Lv 16)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 20)
- <a href="move-lookup.html?q=dragon-breath">Dragon Breath</a> (Lv 24)
- <a href="move-lookup.html?q=ice-fang">Ice Fang</a> (Lv 28)
- <a href="move-lookup.html?q=stomp">Stomp</a> (Lv 33)
- <a href="move-lookup.html?q=super-fang">Super Fang</a> (Lv 35)
- <a href="move-lookup.html?q=stomping-tantrum">Stomping Tantrum</a> (Lv 38)
- <a href="move-lookup.html?q=slam">Slam</a> (Lv 40)
- <a href="move-lookup.html?q=crunch">Crunch</a> (Lv 44)
- <a href="move-lookup.html?q=aqua-jet">Aqua Jet</a> (Lv 48)
- <a href="move-lookup.html?q=fishious-rend">Fishious Rend</a> (Lv 52)
- <a href="move-lookup.html?q=dragon-pulse">Dragon Pulse</a> (Lv 55)
- <a href="move-lookup.html?q=dragon-rush">Dragon Rush</a> (Lv 58)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=mega-kick">Mega Kick</a>
- <a href="move-lookup.html?q=rock-slide">Rock Slide</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
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
